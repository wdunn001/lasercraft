#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! The LaserCraft engine: the integration layer.
//!
//! Every Craft app exposes a headless engine driven by an `engine.execute`
//! control protocol, shared across the suite as `docs/control-protocol.md`. This
//! crate is LaserCraft's engine. It holds the job document and turns it into
//! controller output by composing the vendor-free crates: the document model, the
//! toolpath generator, the motion limits, and a GCode dialect.

use lasercraft_galvo::{GalvoCommand, GalvoParam, MarkRun, lmc, mark_polyline, mark_scanlines};
use lasercraft_gcode::{Dialect, Op, program};
use lasercraft_laser_doc::{CutSetting, FillMode, Geometry, Point2, Project, Shape, ShapeKind};
use lasercraft_motion::MotionConfig;
use lasercraft_raster::{GrayImage, RasterParams, raster_scan};
use lasercraft_toolpath::{
    BBox, Point, apply_tabs, dot_fill_polygon, follow_polyline, hatch_fill_angled,
    hatch_fill_multi, lead_in_segment, lead_out_segment, line_fill, overcut_segments,
    perforate_path, wobble_path,
};

/// Errors the engine can return while executing a command.
#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    /// A command referenced a cut index with no matching setting.
    #[error("no cut setting with index {0}")]
    UnknownCutIndex(u32),
}

/// A control-protocol command. The envelope mirrors the suite's `engine.execute`.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Record the authoring application version.
    SetAppVersion(String),
    /// Add a cut setting to the job. Boxed because a galvo cut setting is large.
    AddCutSetting(Box<CutSetting>),
    /// Add a shape to the job.
    AddShape(Shape),
    /// Remove all shapes and cut settings.
    Clear,
}

/// The headless engine. It owns the job document.
#[derive(Debug, Default)]
pub struct Engine {
    project: Project,
}

impl Engine {
    /// Create an engine with an empty job.
    pub fn new() -> Self {
        Self::default()
    }

    /// The current job document.
    pub fn project(&self) -> &Project {
        &self.project
    }

    /// Execute one control-protocol command against the job.
    pub fn execute(&mut self, command: Command) -> Result<(), EngineError> {
        match command {
            Command::SetAppVersion(version) => self.project.app_version = version,
            Command::AddCutSetting(setting) => self.project.cut_settings.push(*setting),
            Command::AddShape(shape) => {
                if self.project.cut_setting_for(&shape).is_none() {
                    return Err(EngineError::UnknownCutIndex(shape.cut_index));
                }
                self.project.shapes.push(shape);
            }
            Command::Clear => {
                self.project.shapes.clear();
                self.project.cut_settings.clear();
            }
        }
        Ok(())
    }
}

/// Generate controller output for a line-filled region.
///
/// This is the end-to-end slice that crosses every vendor-free crate: the cut
/// setting comes from the document model, [`line_fill`] generates the scan
/// segments, [`MotionConfig`] clamps the feed to the machine limit, and the
/// [`Dialect`] renders the GCode. Each scan line is a rapid to its start with the
/// laser off, then a cut across at the layer power.
pub fn plan_line_fill(
    bbox: &BBox,
    setting: &CutSetting,
    motion: &MotionConfig,
    interval_mm: f64,
    dialect: &dyn Dialect,
) -> String {
    let feed = motion.clamp_feed(setting.speed_mm_s);
    let segments = line_fill(bbox, interval_mm);
    let mut ops = Vec::with_capacity(segments.len() * 2 * setting.num_passes as usize);
    for _ in 0..setting.num_passes {
        for segment in &segments {
            ops.push(Op::RapidTo {
                x: segment.start.x,
                y: segment.start.y,
            });
            ops.push(Op::CutTo {
                x: segment.end.x,
                y: segment.end.y,
                feed_mm_s: feed,
                power: setting.power_max,
            });
        }
    }
    program(dialect, &ops)
}

/// Generate controller output for an entire project.
///
/// Each shape with polyline geometry is cut in order: a rapid move to the first
/// point with the laser off, then a cut along every segment at the layer feed and
/// power. Closed polylines return to their start. Shapes with no geometry or no
/// resolvable cut setting are skipped. This is the vector path of the pipeline,
/// alongside [`plan_line_fill`] for filled regions.
pub fn plan_project(project: &Project, motion: &MotionConfig, dialect: &dyn Dialect) -> String {
    // A bezier path flattens to this many chords per cubic before toolpathing.
    const CURVE_STEPS: u32 = 24;
    // Collect the drawable shapes as toolpathed segment runs.
    struct Run {
        segments: Vec<lasercraft_toolpath::Segment>,
        points: Vec<Point>,
        closed: bool,
        feed: f64,
        power: f64,
        passes: u32,
        fill: bool,
        overscan: f64,
    }
    let mut runs: Vec<Run> = Vec::new();
    // Flood-fill shapes are gathered by layer and hatched together after the loop.
    let mut flood: std::collections::BTreeMap<u32, Vec<Vec<Point>>> =
        std::collections::BTreeMap::new();
    // Dot-mode fills produce pulses, not segments, so collect them for the end.
    let mut dot_jobs: Vec<(Vec<Point>, f64, f64)> = Vec::new();
    for shape in &project.shapes {
        let (points, closed) = match &shape.geometry {
            Geometry::Polyline { points, closed } => (points.clone(), *closed),
            Geometry::Path(path) => (path.flatten(CURVE_STEPS), path.closed),
            Geometry::Empty => continue,
        };
        let Some(setting) = project.cut_setting_for(shape) else {
            continue;
        };
        let path: Vec<Point> = points.iter().map(|p| Point { x: p.x, y: p.y }).collect();
        let flood_on = matches!(
            setting.galvo.as_ref(),
            Some(g) if g.mode == FillMode::Fill && g.fill.flood_fill
        );
        if closed && flood_on {
            flood.entry(shape.cut_index).or_default().push(path);
            continue;
        }
        // Dot mode pulses a grid of dots inside a closed fill instead of scanning.
        let dot_on = matches!(
            setting.galvo.as_ref(),
            Some(g) if g.mode == FillMode::Fill && g.fill.dot_mode
        );
        if closed && dot_on {
            if let Some(g) = setting.galvo.as_ref() {
                let dots = dot_fill_polygon(&path, g.fill.line_interval_mm);
                if !dots.is_empty() {
                    dot_jobs.push((dots, setting.power_max, g.fill.dot_time_ms / 1000.0));
                }
            }
            continue;
        }
        let is_fill =
            closed && matches!(setting.galvo.as_ref(), Some(g) if g.mode == FillMode::Fill);
        let overscan = setting
            .galvo
            .as_ref()
            .map(|g| g.fill.overscan_mm)
            .unwrap_or(0.0);
        // A Fill layer hatches the closed interior with scan lines; every other mode
        // follows the outline. This makes engrave fills work on G-code machines, not
        // just galvo.
        let segments = match (closed, setting.galvo.as_ref()) {
            (true, Some(g)) if g.mode == FillMode::Fill => hatch_fill_angled(
                &path,
                g.fill.line_interval_mm,
                g.fill.hatch_angle_deg,
                g.fill.cross_hatch,
                g.fill.bidirectional,
            ),
            _ => {
                // Line mode: compose the outline with lead-in, perforation,
                // overcut, and lead-out when the layer asks for them.
                let line = setting.galvo.as_ref().map(|g| &g.line);
                // Kerf offset: shift a closed cut outward or inward by the beam-width
                // compensation, so the finished part holds its drawn size. An offset
                // that collapses falls back to the original outline.
                let kerf = line.map(|l| l.kerf_offset_mm).unwrap_or(0.0);
                let contours: Vec<Vec<Point>> = if closed && kerf.abs() > 1e-9 && path.len() >= 3 {
                    let tuples: Vec<(f64, f64)> = path.iter().map(|p| (p.x, p.y)).collect();
                    let rings = lasercraft_geomops::offset(&tuples, kerf);
                    if rings.is_empty() {
                        vec![path.clone()]
                    } else {
                        rings
                            .into_iter()
                            .map(|r| r.into_iter().map(|(x, y)| Point { x, y }).collect())
                            .collect()
                    }
                } else {
                    vec![path.clone()]
                };
                let tabs = setting.galvo.as_ref().map(|g| &g.tabs);
                let mut segs = Vec::new();
                for contour in &contours {
                    if let Some(li) = line.and_then(|l| lead_in_segment(contour, l.lead_in_mm)) {
                        segs.push(li);
                    }
                    let mut outline = match line {
                        Some(l) if l.perforation => perforate_path(
                            contour,
                            closed,
                            l.perforation_cut_mm,
                            l.perforation_skip_mm,
                        ),
                        _ => follow_polyline(contour, closed),
                    };
                    // Tabs leave small uncut bridges that hold the part in the sheet.
                    if let Some(t) = tabs.filter(|t| closed && t.enabled && t.tabs_per_shape > 0) {
                        outline = apply_tabs(&outline, t.tabs_per_shape, t.size_mm);
                    }
                    segs.extend(outline);
                    if let Some(l) = line {
                        segs.extend(overcut_segments(contour, closed, l.overcut_mm));
                    }
                    if let Some(lo) =
                        line.and_then(|l| lead_out_segment(contour, closed, l.lead_out_mm))
                    {
                        segs.push(lo);
                    }
                }
                segs
            }
        };
        if segments.is_empty() {
            continue;
        }
        runs.push(Run {
            segments,
            points: path,
            closed,
            feed: motion.clamp_feed(setting.speed_mm_s),
            power: setting.power_max,
            passes: setting.num_passes,
            fill: is_fill,
            overscan,
        });
    }
    // Flood fill: hatch every shape on a layer as one field, one run per layer.
    for (ci, polys) in flood {
        let Some(setting) = project.cut_settings.iter().find(|s| s.index == ci) else {
            continue;
        };
        let Some(g) = setting.galvo.as_ref() else {
            continue;
        };
        let segments = hatch_fill_multi(
            &polys,
            g.fill.line_interval_mm,
            g.fill.hatch_angle_deg,
            g.fill.cross_hatch,
            g.fill.bidirectional,
        );
        if segments.is_empty() {
            continue;
        }
        let points = polys.into_iter().next().unwrap_or_default();
        runs.push(Run {
            segments,
            points,
            closed: true,
            feed: motion.clamp_feed(setting.speed_mm_s),
            power: setting.power_max,
            passes: setting.num_passes,
            fill: true,
            overscan: g.fill.overscan_mm,
        });
    }
    // Order the runs so an enclosed shape cuts before its container, and travel
    // between shapes stays short.
    let starts: Vec<Point> = runs
        .iter()
        .filter_map(|r| r.segments.first().map(|s| s.start))
        .collect();
    let run_points: Vec<Vec<Point>> = runs.iter().map(|r| r.points.clone()).collect();
    let run_closed: Vec<bool> = runs.iter().map(|r| r.closed).collect();
    let order = nested_order(&run_points, &run_closed, &starts);
    let mut ops = Vec::new();
    for &index in &order {
        let Some(run) = runs.get(index) else { continue };
        for _ in 0..run.passes {
            // Rapid to the start of any segment that does not continue from the last
            // one, so disconnected hatch lines travel with the beam off.
            // Overscan runs each fill scan line past its ends with the beam off, so
            // the head reaches speed before it enters the engraving.
            let overscan = if run.fill { run.overscan } else { 0.0 };
            let mut cur: Option<Point> = None;
            for segment in &run.segments {
                let (dx, dy) = (
                    segment.end.x - segment.start.x,
                    segment.end.y - segment.start.y,
                );
                let len = dx.hypot(dy);
                let (ux, uy) = if len > 1e-9 {
                    (dx / len, dy / len)
                } else {
                    (0.0, 0.0)
                };
                let gap = match cur {
                    Some(p) => {
                        (p.x - segment.start.x).abs() > 1e-6 || (p.y - segment.start.y).abs() > 1e-6
                    }
                    None => true,
                };
                if gap && overscan > 0.0 && len > 1e-9 {
                    // Rapid to a point before the line, then feed in with the beam off.
                    ops.push(Op::RapidTo {
                        x: segment.start.x - ux * overscan,
                        y: segment.start.y - uy * overscan,
                    });
                    ops.push(Op::CutTo {
                        x: segment.start.x,
                        y: segment.start.y,
                        feed_mm_s: run.feed,
                        power: 0.0,
                    });
                } else if gap {
                    ops.push(Op::RapidTo {
                        x: segment.start.x,
                        y: segment.start.y,
                    });
                }
                ops.push(Op::CutTo {
                    x: segment.end.x,
                    y: segment.end.y,
                    feed_mm_s: run.feed,
                    power: run.power,
                });
                if overscan > 0.0 && len > 1e-9 {
                    // Continue past the end with the beam off.
                    let post = Point {
                        x: segment.end.x + ux * overscan,
                        y: segment.end.y + uy * overscan,
                    };
                    ops.push(Op::CutTo {
                        x: post.x,
                        y: post.y,
                        feed_mm_s: run.feed,
                        power: 0.0,
                    });
                    cur = Some(post);
                } else {
                    cur = Some(segment.end);
                }
            }
        }
    }
    // Dot-mode fills: one pulse per grid dot, beam off between dots.
    for (dots, power, dwell_s) in dot_jobs {
        for d in dots {
            ops.push(Op::DotMark {
                x: d.x,
                y: d.y,
                power,
                dwell_s,
            });
        }
    }
    program(dialect, &ops)
}

/// Render a project to an SVG preview string.
///
/// Each shape is drawn as a path, coloured by its cut index so layers are
/// distinguishable, over a dark bed. The Y axis is flipped so the preview matches
/// the laser's bottom-left origin. This is a viewing aid and does not drive the
/// machine.
pub fn project_to_svg(project: &Project) -> String {
    use std::fmt::Write as _;
    const PALETTE: &[&str] = &[
        "#e23", "#2a7", "#39f", "#fb0", "#b5f", "#0cc", "#f70", "#7d2",
    ];
    const CURVE_STEPS: u32 = 24;
    let (min, max) = project
        .bounds()
        .unwrap_or((Point2 { x: 0.0, y: 0.0 }, Point2 { x: 100.0, y: 100.0 }));
    let pad = ((max.x - min.x).max(max.y - min.y) * 0.05).max(1.0);
    let (x0, y0) = (min.x - pad, min.y - pad);
    let w = (max.x - min.x + 2.0 * pad).max(1.0);
    let h = (max.y - min.y + 2.0 * pad).max(1.0);
    let stroke = (w.max(h) / 300.0).max(0.05);

    let mut svg = String::new();
    let _ = write!(
        svg,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{x0:.3} {y0:.3} {w:.3} {h:.3}\" width=\"640\">"
    );
    let _ = write!(
        svg,
        "<rect x=\"{x0:.3}\" y=\"{y0:.3}\" width=\"{w:.3}\" height=\"{h:.3}\" fill=\"#111\"/>"
    );
    // Flip Y so the bottom-left origin reads correctly.
    let _ = write!(
        svg,
        "<g transform=\"translate(0 {:.3}) scale(1 -1)\">",
        2.0 * y0 + h
    );
    for shape in &project.shapes {
        let (points, closed) = match &shape.geometry {
            Geometry::Polyline { points, closed } => (points.clone(), *closed),
            Geometry::Path(path) => (path.flatten(CURVE_STEPS), path.closed),
            Geometry::Empty => continue,
        };
        if points.is_empty() {
            continue;
        }
        let color = PALETTE[(shape.cut_index as usize) % PALETTE.len()];
        let mut d = String::new();
        for (i, p) in points.iter().enumerate() {
            let cmd = if i == 0 { 'M' } else { 'L' };
            let _ = write!(d, "{cmd}{:.3} {:.3} ", p.x, p.y);
        }
        if closed {
            d.push('Z');
        }
        let _ = write!(
            svg,
            "<path d=\"{d}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"{stroke:.3}\"/>"
        );
    }
    svg.push_str("</g></svg>");
    svg
}

/// A material-test grid: columns vary speed, rows vary power.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MaterialTestSpec {
    /// Number of columns, each a speed step.
    pub columns: u32,
    /// Number of rows, each a power step.
    pub rows: u32,
    /// The side length of each test square in millimetres.
    pub cell_mm: f64,
    /// The gap between squares in millimetres.
    pub gap_mm: f64,
    /// The speed of the first column in millimetres per second.
    pub speed_min: f64,
    /// The speed of the last column.
    pub speed_max: f64,
    /// The power of the first row as a percentage.
    pub power_min: f64,
    /// The power of the last row.
    pub power_max: f64,
}

/// Build a material-test project: a grid of squares, each cut at its own speed and
/// power, for dialing in settings on a new material. Columns step speed left to
/// right, rows step power bottom to top. Each square becomes a shape with its own
/// cut setting, so the grid cuts and estimates through the normal pipeline.
pub fn material_test(spec: &MaterialTestSpec) -> Project {
    fn fraction(index: u32, count: u32) -> f64 {
        if count <= 1 {
            0.0
        } else {
            f64::from(index) / f64::from(count - 1)
        }
    }
    fn lerp(a: f64, b: f64, t: f64) -> f64 {
        a + (b - a) * t
    }
    let mut project = Project::new();
    project.app_version = "material-test".to_string();
    let step = spec.cell_mm + spec.gap_mm;
    for row in 0..spec.rows {
        for col in 0..spec.columns {
            let index = row * spec.columns + col;
            let speed = lerp(spec.speed_min, spec.speed_max, fraction(col, spec.columns));
            let power = lerp(spec.power_min, spec.power_max, fraction(row, spec.rows));
            let x0 = f64::from(col) * step;
            let y0 = f64::from(row) * step;
            let c = spec.cell_mm;
            project.cut_settings.push(CutSetting {
                index,
                name: format!("{speed:.0} mm/s {power:.0}%"),
                speed_mm_s: speed,
                power_min: 0.0,
                power_max: power,
                num_passes: 1,
                air_assist: true,
                galvo: None,
            });
            project.shapes.push(Shape {
                kind: ShapeKind::Rect,
                cut_index: index,
                geometry: Geometry::Polyline {
                    points: vec![
                        Point2 { x: x0, y: y0 },
                        Point2 { x: x0 + c, y: y0 },
                        Point2 {
                            x: x0 + c,
                            y: y0 + c,
                        },
                        Point2 { x: x0, y: y0 + c },
                    ],
                    closed: true,
                },
            });
        }
    }
    project
}

/// The unsigned area of a closed polygon by the shoelace sum.
fn polygon_area(poly: &[Point]) -> f64 {
    let n = poly.len();
    if n < 3 {
        return 0.0;
    }
    let mut sum = 0.0;
    let mut j = n - 1;
    for i in 0..n {
        sum += (poly[j].x + poly[i].x) * (poly[j].y - poly[i].y);
        j = i;
    }
    (sum / 2.0).abs()
}

/// Whether `point` lies inside the closed ring `poly`, by ray casting.
fn point_in_polygon(point: Point, poly: &[Point]) -> bool {
    let n = poly.len();
    if n < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = n - 1;
    for i in 0..n {
        let a = poly[i];
        let b = poly[j];
        if (a.y > point.y) != (b.y > point.y)
            && point.x < (b.x - a.x) * (point.y - a.y) / (b.y - a.y) + a.x
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}

/// Order runs so an enclosed shape cuts before the shape that encloses it.
///
/// A part that sits inside another, a hole in a plate or a letter counter, must be
/// cut while the outer shape still holds the material. Otherwise the outer cut
/// frees the piece and the inner cut lands on something that has already dropped.
/// A run waits until every run it contains is done, and among the ready runs the
/// nearest to the laser's current position goes next, so travel stays short.
// The loops index several parallel arrays (points, areas, children, visited,
// starts) by the same i, so a range loop reads clearer than zipped iterators.
#[allow(clippy::needless_range_loop)]
fn nested_order(points: &[Vec<Point>], closed: &[bool], starts: &[Point]) -> Vec<usize> {
    let n = starts.len();
    let areas: Vec<f64> = points.iter().map(|p| polygon_area(p)).collect();
    // children[i] are the runs that run i encloses, so i must wait for them.
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        if !closed.get(i).copied().unwrap_or(false) {
            continue;
        }
        let Some(outer) = points.get(i) else { continue };
        for j in 0..n {
            if i == j {
                continue;
            }
            let rep = points.get(j).and_then(|p| p.first()).copied();
            let (Some(rep), Some(&outer_area), Some(&inner_area)) =
                (rep, areas.get(i), areas.get(j))
            else {
                continue;
            };
            if outer_area > inner_area && point_in_polygon(rep, outer) {
                children[i].push(j);
            }
        }
    }
    let mut visited = vec![false; n];
    let mut order = Vec::with_capacity(n);
    let mut current = Point { x: 0.0, y: 0.0 };
    for _ in 0..n {
        let mut best: Option<usize> = None;
        let mut best_distance = f64::INFINITY;
        for i in 0..n {
            if visited[i] || !children[i].iter().all(|&c| visited[c]) {
                continue;
            }
            let Some(start) = starts.get(i) else { continue };
            let distance = (start.x - current.x).hypot(start.y - current.y);
            if distance < best_distance {
                best_distance = distance;
                best = Some(i);
            }
        }
        // A containment cycle should not happen, but fall back to the nearest
        // unvisited run so the walk always makes progress.
        if best.is_none() {
            for i in 0..n {
                if visited[i] {
                    continue;
                }
                let Some(start) = starts.get(i) else { continue };
                let distance = (start.x - current.x).hypot(start.y - current.y);
                if distance < best_distance {
                    best_distance = distance;
                    best = Some(i);
                }
            }
        }
        if let Some(i) = best {
            visited[i] = true;
            order.push(i);
            if let Some(start) = starts.get(i) {
                current = *start;
            }
        }
    }
    order
}

/// Order points by a greedy nearest-neighbour walk from the origin.
///
/// This shortens the rapid travel between shapes, the air-time that LightBurn
/// leaves on the table when it cuts in document order. Nearest-neighbour is a
/// cheap heuristic that captures most of the saving.
pub fn optimize_order(starts: &[Point]) -> Vec<usize> {
    let count = starts.len();
    let mut visited = vec![false; count];
    let mut order = Vec::with_capacity(count);
    let mut current = Point { x: 0.0, y: 0.0 };
    for _ in 0..count {
        let mut best: Option<usize> = None;
        let mut best_distance = f64::INFINITY;
        for (i, start) in starts.iter().enumerate() {
            if visited[i] {
                continue;
            }
            let distance = (start.x - current.x).hypot(start.y - current.y);
            if distance < best_distance {
                best_distance = distance;
                best = Some(i);
            }
        }
        if let Some(i) = best {
            visited[i] = true;
            order.push(i);
            current = starts[i];
        }
    }
    order
}

/// Generate galvo marking commands that cut every polyline shape in a project.
///
/// This is the galvo analogue of [`plan_project`], for fiber and galvo machines
/// like the EZCAD2 path. Each shape is flattened and marked at its layer power and
/// the given pulse frequency. The board-specific byte encoding serializes the
/// returned commands.
pub fn plan_galvo_project(
    project: &Project,
    param: &GalvoParam,
    frequency_hz: u32,
) -> Vec<GalvoCommand> {
    const CURVE_STEPS: u32 = 24;
    let mut commands = Vec::new();
    // Order the shapes so an enclosed shape marks before the shape that encloses
    // it, the same containment rule the GCode planner uses, applied to the galvo
    // path. A fiber job then marks a hole or counter before its outline frees it.
    let mut drawable: Vec<usize> = Vec::new();
    let mut ord_points: Vec<Vec<Point>> = Vec::new();
    let mut ord_closed: Vec<bool> = Vec::new();
    let mut ord_start: Vec<Point> = Vec::new();
    for (si, shape) in project.shapes.iter().enumerate() {
        let (points, closed) = match &shape.geometry {
            Geometry::Polyline { points, closed } => (points.clone(), *closed),
            Geometry::Path(path) => (path.flatten(CURVE_STEPS), path.closed),
            Geometry::Empty => continue,
        };
        if project.cut_setting_for(shape).is_none() {
            continue;
        }
        let pts: Vec<Point> = points.iter().map(|p| Point { x: p.x, y: p.y }).collect();
        let start = pts.first().copied().unwrap_or(Point { x: 0.0, y: 0.0 });
        drawable.push(si);
        ord_start.push(start);
        ord_closed.push(closed);
        ord_points.push(pts);
    }
    let order = nested_order(&ord_points, &ord_closed, &ord_start);
    for &oi in &order {
        let Some(shape) = drawable.get(oi).and_then(|&si| project.shapes.get(si)) else {
            continue;
        };
        let (points, closed) = match &shape.geometry {
            Geometry::Polyline { points, closed } => (points.clone(), *closed),
            Geometry::Path(path) => (path.flatten(CURVE_STEPS), path.closed),
            Geometry::Empty => continue,
        };
        let Some(setting) = project.cut_setting_for(shape) else {
            continue;
        };
        // A Concentric layer insets the outline in interval steps, the true offset
        // fill. The shared `geomops::concentric` runs the craft-pathops offset, the
        // same code the browser editor uses. The outline itself marks first, then
        // each inset ring.
        if let Some(galvo) = &setting.galvo
            && galvo.mode == FillMode::Concentric
            && closed
        {
            let outline: Vec<(f64, f64)> = points.iter().map(|p| (p.x, p.y)).collect();
            let interval = galvo.fill.line_interval_mm.max(0.01);
            commands.extend(mark_polyline(
                param,
                &outline,
                true,
                setting.power_max,
                frequency_hz,
            ));
            for ring in lasercraft_geomops::concentric(&outline, interval, 500) {
                commands.extend(mark_polyline(
                    param,
                    &ring,
                    true,
                    setting.power_max,
                    frequency_hz,
                ));
            }
            continue;
        }
        // A Fill layer hatches the shape interior with scan lines, clipped to the
        // outline.
        if let Some(galvo) = &setting.galvo
            && galvo.mode == FillMode::Fill
        {
            let poly: Vec<Point> = points.iter().map(|p| Point { x: p.x, y: p.y }).collect();
            let segments = hatch_fill_angled(
                &poly,
                galvo.fill.line_interval_mm,
                galvo.fill.hatch_angle_deg,
                galvo.fill.cross_hatch,
                galvo.fill.bidirectional,
            );
            commands.push(GalvoCommand::SetPower {
                percent: setting.power_max.clamp(0.0, 100.0),
            });
            commands.push(GalvoCommand::SetFrequency { hz: frequency_hz });
            for seg in &segments {
                let (sx, sy) = param.logic_to_device(seg.start.x, seg.start.y);
                let (ex, ey) = param.logic_to_device(seg.end.x, seg.end.y);
                commands.push(GalvoCommand::Jump { x: sx, y: sy });
                commands.push(GalvoCommand::Mark { x: ex, y: ey });
            }
            continue;
        }
        // A layer with wobble enabled marks a widened, dense circle path instead.
        if let Some(galvo) = &setting.galvo
            && galvo.wobble.enabled
        {
            let mut pts: Vec<Point> = points.iter().map(|p| Point { x: p.x, y: p.y }).collect();
            if closed && let Some(first) = pts.first().copied() {
                pts.push(first);
            }
            let wobbled = wobble_path(&pts, galvo.wobble.step_mm, galvo.wobble.size_mm);
            let wpath: Vec<(f64, f64)> = wobbled.iter().map(|p| (p.x, p.y)).collect();
            commands.extend(mark_polyline(
                param,
                &wpath,
                false,
                setting.power_max,
                frequency_hz,
            ));
            continue;
        }
        let path: Vec<(f64, f64)> = points.iter().map(|p| (p.x, p.y)).collect();
        commands.extend(mark_polyline(
            param,
            &path,
            closed,
            setting.power_max,
            frequency_hz,
        ));
    }
    commands
}

/// Generate galvo marking commands to raster-engrave an image on a galvo machine.
pub fn plan_galvo_raster(
    image: &GrayImage,
    params: &RasterParams,
    param: &GalvoParam,
    frequency_hz: u32,
) -> Vec<GalvoCommand> {
    let runs: Vec<MarkRun> = raster_scan(image, params)
        .iter()
        .map(|r| MarkRun {
            y_mm: r.y_mm,
            x_start_mm: r.x_start_mm,
            x_end_mm: r.x_end_mm,
            power_percent: r.power,
        })
        .collect();
    mark_scanlines(param, &runs, frequency_hz)
}

/// Plan an entire galvo project as BJJCZ LMC board bytes.
///
/// This is the galvo analogue of [`plan_project`] carried all the way to the
/// wire. It maps every shape through the field with [`plan_galvo_project`], then
/// serialises the marking commands into the LMC list format for the bulk write to
/// the controller. `mark_speed` and `power_ratio` are the board's own raw values,
/// which the device profile and its calibration supply.
pub fn plan_galvo_lmc(
    project: &Project,
    param: &GalvoParam,
    frequency_hz: u32,
    mark_speed: u16,
    power_ratio: u16,
) -> Vec<u8> {
    let commands = plan_galvo_project(project, param, frequency_hz);
    lmc::compile(&commands, mark_speed, power_ratio)
}

/// Estimate the cutting time of a project in seconds.
///
/// Each shape is flattened and every segment is timed with the trapezoidal
/// acceleration model in [`MotionConfig::move_time`], then multiplied by the
/// layer's pass count. This accounts for acceleration on short segments, which is
/// where a naive length-over-feed estimate is most wrong. Rapid travel between
/// shapes is not counted, so this is a lower bound on wall-clock time.
pub fn estimate_job_time(project: &Project, motion: &MotionConfig) -> f64 {
    const CURVE_STEPS: u32 = 24;
    let mut seconds = 0.0;
    for shape in &project.shapes {
        let (points, closed) = match &shape.geometry {
            Geometry::Polyline { points, closed } => (points.clone(), *closed),
            Geometry::Path(path) => (path.flatten(CURVE_STEPS), path.closed),
            Geometry::Empty => continue,
        };
        let Some(setting) = project.cut_setting_for(shape) else {
            continue;
        };
        let path: Vec<Point> = points.iter().map(|p| Point { x: p.x, y: p.y }).collect();
        let per_pass: f64 = follow_polyline(&path, closed)
            .iter()
            .map(|seg| {
                let dx = seg.end.x - seg.start.x;
                let dy = seg.end.y - seg.start.y;
                motion.move_time(dx.hypot(dy), setting.speed_mm_s)
            })
            .sum();
        seconds += per_pass * f64::from(setting.num_passes);
    }
    seconds
}

/// Generate a framing pass that traces the job's bounding box.
///
/// Framing moves the head around the extent of the job with the laser off so the
/// operator can position the material before cutting. The head rapids around the
/// four corners and back to the start. A project with no geometry frames nothing.
pub fn plan_frame(project: &Project, dialect: &dyn Dialect) -> String {
    let Some((min, max)) = project.bounds() else {
        let mut empty = dialect.header();
        empty.push_str(&dialect.footer());
        return empty;
    };
    let corners = [
        (min.x, min.y),
        (max.x, min.y),
        (max.x, max.y),
        (min.x, max.y),
        (min.x, min.y),
    ];
    let ops: Vec<Op> = corners.iter().map(|&(x, y)| Op::RapidTo { x, y }).collect();
    program(dialect, &ops)
}

/// How the framing pass traces the job. LightBurn "Bounds", "Hull", "Contour".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FrameStyle {
    /// The bounding rectangle around everything. LightBurn "Bounds".
    #[default]
    Bounds,
    /// The convex hull of all geometry, a tight wrap. LightBurn "Hull".
    Hull,
    /// The outline of every shape in turn. LightBurn "Contour".
    Contour,
}

/// The flattened vertices of one shape, in document coordinates.
fn shape_points(shape: &Shape) -> Vec<Point> {
    const CURVE_STEPS: u32 = 24;
    let pts = match &shape.geometry {
        Geometry::Polyline { points, .. } => points.clone(),
        Geometry::Path(path) => path.flatten(CURVE_STEPS),
        Geometry::Empty => return Vec::new(),
    };
    pts.iter().map(|p| Point { x: p.x, y: p.y }).collect()
}

/// The convex hull of a point set, counter-clockwise, by Andrew's monotone
/// chain. Returns the input when there are fewer than three distinct points.
fn convex_hull(points: &[Point]) -> Vec<Point> {
    let mut pts: Vec<Point> = points.to_vec();
    pts.sort_by(|a, b| {
        a.x.partial_cmp(&b.x)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
    });
    pts.dedup_by(|a, b| a.x == b.x && a.y == b.y);
    if pts.len() < 3 {
        return pts;
    }
    let cross =
        |o: Point, a: Point, b: Point| (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
    let build = |iter: &mut dyn Iterator<Item = Point>| -> Vec<Point> {
        let mut chain: Vec<Point> = Vec::new();
        for p in iter {
            while chain.len() >= 2 {
                let n = chain.len();
                let (Some(&a), Some(&b)) = (chain.get(n - 2), chain.get(n - 1)) else {
                    break;
                };
                if cross(a, b, p) <= 0.0 {
                    chain.pop();
                } else {
                    break;
                }
            }
            chain.push(p);
        }
        chain.pop();
        chain
    };
    let mut lower = build(&mut pts.iter().copied());
    let upper = build(&mut pts.iter().rev().copied());
    lower.extend(upper);
    lower
}

/// Plan a framing pass in the given style.
///
/// Bounds traces the bounding rectangle. Hull traces the convex hull of all
/// geometry. Contour traces each shape's own outline. Every move is a rapid with
/// the laser off, so the operator sees where the job lands before marking.
pub fn plan_frame_styled(project: &Project, style: FrameStyle, dialect: &dyn Dialect) -> String {
    let empty = || {
        let mut s = dialect.header();
        s.push_str(&dialect.footer());
        s
    };
    match style {
        FrameStyle::Bounds => plan_frame(project, dialect),
        FrameStyle::Hull => {
            let points: Vec<Point> = project.shapes.iter().flat_map(shape_points).collect();
            let hull = convex_hull(&points);
            let Some(&first) = hull.first() else {
                return empty();
            };
            if hull.len() < 2 {
                return empty();
            }
            let mut ops: Vec<Op> = hull
                .iter()
                .map(|p| Op::RapidTo { x: p.x, y: p.y })
                .collect();
            ops.push(Op::RapidTo {
                x: first.x,
                y: first.y,
            });
            program(dialect, &ops)
        }
        FrameStyle::Contour => {
            let mut ops: Vec<Op> = Vec::new();
            for shape in &project.shapes {
                let pts = shape_points(shape);
                let Some(&first) = pts.first() else { continue };
                for p in &pts {
                    ops.push(Op::RapidTo { x: p.x, y: p.y });
                }
                ops.push(Op::RapidTo {
                    x: first.x,
                    y: first.y,
                });
            }
            if ops.is_empty() {
                return empty();
            }
            program(dialect, &ops)
        }
    }
}

/// Generate controller output to raster-engrave an image.
///
/// Each constant-power scan run becomes a rapid to its start with the laser off,
/// then a cut across the span at the run power and the configured engrave feed.
/// This is the image-engraving path, alongside the vector and line-fill paths.
pub fn plan_raster(
    image: &GrayImage,
    params: &RasterParams,
    motion: &MotionConfig,
    dialect: &dyn Dialect,
) -> String {
    let feed = motion.clamp_feed(params.speed_mm_s);
    let runs = raster_scan(image, params);
    let mut ops = Vec::with_capacity(runs.len() * 2);
    for run in &runs {
        ops.push(Op::RapidTo {
            x: run.x_start_mm,
            y: run.y_mm,
        });
        ops.push(Op::CutTo {
            x: run.x_end_mm,
            y: run.y_mm,
            feed_mm_s: feed,
            power: run.power,
        });
    }
    program(dialect, &ops)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lasercraft_gcode::GCodeDialect;

    fn cut() -> CutSetting {
        CutSetting {
            index: 0,
            name: "Cut".to_string(),
            speed_mm_s: 20.0,
            power_min: 10.0,
            power_max: 80.0,
            num_passes: 1,
            air_assist: true,
            galvo: None,
        }
    }

    #[test]
    fn adding_a_shape_needs_a_known_cut_index() {
        let mut engine = Engine::new();
        let shape = Shape {
            kind: ShapeKind::Path,
            cut_index: 0,
            geometry: Geometry::Empty,
        };
        assert!(matches!(
            engine.execute(Command::AddShape(shape.clone())),
            Err(EngineError::UnknownCutIndex(0))
        ));
        assert!(
            engine
                .execute(Command::AddCutSetting(Box::new(cut())))
                .is_ok()
        );
        assert!(engine.execute(Command::AddShape(shape)).is_ok());
        assert_eq!(engine.project().shapes.len(), 1);
    }

    #[test]
    fn framing_styles_trace_bounds_hull_and_contour() {
        let dialect = GCodeDialect::grbl();
        let mut project = Project::new();
        project.cut_settings.push(cut());
        // A triangle and a separate square.
        project.shapes.push(Shape {
            kind: ShapeKind::Path,
            cut_index: 0,
            geometry: Geometry::Polyline {
                points: vec![
                    Point2 { x: 0.0, y: 0.0 },
                    Point2 { x: 10.0, y: 0.0 },
                    Point2 { x: 5.0, y: 8.0 },
                ],
                closed: true,
            },
        });
        project.shapes.push(Shape {
            kind: ShapeKind::Rect,
            cut_index: 0,
            geometry: Geometry::Polyline {
                points: vec![
                    Point2 { x: 20.0, y: 0.0 },
                    Point2 { x: 30.0, y: 0.0 },
                    Point2 { x: 30.0, y: 8.0 },
                    Point2 { x: 20.0, y: 8.0 },
                ],
                closed: true,
            },
        });
        // Each rapid move writes one X coordinate, so counting X tracks the move count.
        let moves = |s: &str| s.matches('X').count();
        // Bounds frames the overall rectangle: four corners and a close.
        let bounds = plan_frame_styled(&project, FrameStyle::Bounds, &dialect);
        assert!(moves(&bounds) >= 5, "bounds moves {}", moves(&bounds));
        assert!(bounds.contains("X30") && bounds.contains("Y8"));
        // Hull wraps all the outer points in one loop.
        let hull = plan_frame_styled(&project, FrameStyle::Hull, &dialect);
        assert!(moves(&hull) >= 4, "hull moves {}", moves(&hull));
        // Contour traces both shapes, so it has more moves than the single bounds loop.
        let contour = plan_frame_styled(&project, FrameStyle::Contour, &dialect);
        assert!(
            moves(&contour) > moves(&bounds),
            "contour {} bounds {}",
            moves(&contour),
            moves(&bounds)
        );
    }

    #[test]
    fn a_galvo_project_compiles_to_lmc_chunks() {
        let mut project = Project::new();
        project.cut_settings.push(cut());
        project.shapes.push(Shape {
            kind: ShapeKind::Path,
            cut_index: 0,
            geometry: Geometry::Polyline {
                points: vec![
                    Point2 { x: -10.0, y: -10.0 },
                    Point2 { x: 10.0, y: -10.0 },
                    Point2 { x: 10.0, y: 10.0 },
                ],
                closed: true,
            },
        });
        let param = GalvoParam {
            field_size_mm: 110.0,
            resolution_bits: 16,
        };
        let bytes = plan_galvo_lmc(&project, &param, 30_000, 100, 0x0800);
        assert!(!bytes.is_empty());
        // The buffer is whole 0xC00 list chunks.
        assert_eq!(bytes.len() % 0xC00, 0);
        // It opens with the marking-speed command 0x800C.
        assert_eq!(&bytes[0..2], &0x800Cu16.to_le_bytes());
    }

    #[test]
    fn a_wobble_layer_densifies_the_galvo_path() {
        use lasercraft_laser_doc::{GalvoSettings, Wobble};
        let param = GalvoParam {
            field_size_mm: 110.0,
            resolution_bits: 16,
        };
        let triangle = || Geometry::Polyline {
            points: vec![
                Point2 { x: -10.0, y: -10.0 },
                Point2 { x: 10.0, y: -10.0 },
                Point2 { x: 5.0, y: 8.0 },
            ],
            closed: true,
        };
        let mut plain = Project::new();
        plain.cut_settings.push(cut());
        plain.shapes.push(Shape {
            kind: ShapeKind::Path,
            cut_index: 0,
            geometry: triangle(),
        });
        let plain_cmds = plan_galvo_project(&plain, &param, 30_000);

        let mut wobble_setting = cut();
        wobble_setting.galvo = Some(GalvoSettings {
            wobble: Wobble {
                enabled: true,
                step_mm: 1.0,
                size_mm: 0.5,
            },
            ..GalvoSettings::default()
        });
        let mut wobbled = Project::new();
        wobbled.cut_settings.push(wobble_setting);
        wobbled.shapes.push(Shape {
            kind: ShapeKind::Path,
            cut_index: 0,
            geometry: triangle(),
        });
        let wobble_cmds = plan_galvo_project(&wobbled, &param, 30_000);

        // Wobble traces many circles, so it yields far more marking commands.
        assert!(
            wobble_cmds.len() > plain_cmds.len() * 5,
            "wobble {} plain {}",
            wobble_cmds.len(),
            plain_cmds.len()
        );
    }

    #[test]
    fn a_fill_layer_hatches_the_shape() {
        use lasercraft_laser_doc::{FillParams, GalvoSettings};
        let param = GalvoParam {
            field_size_mm: 110.0,
            resolution_bits: 16,
        };
        let square = || Geometry::Polyline {
            points: vec![
                Point2 { x: -10.0, y: -10.0 },
                Point2 { x: 10.0, y: -10.0 },
                Point2 { x: 10.0, y: 10.0 },
                Point2 { x: -10.0, y: 10.0 },
            ],
            closed: true,
        };
        let mut outline = Project::new();
        outline.cut_settings.push(cut());
        outline.shapes.push(Shape {
            kind: ShapeKind::Rect,
            cut_index: 0,
            geometry: square(),
        });
        let outline_cmds = plan_galvo_project(&outline, &param, 30_000);

        let mut fill_setting = cut();
        fill_setting.galvo = Some(GalvoSettings {
            mode: FillMode::Fill,
            fill: FillParams {
                line_interval_mm: 1.0,
                ..FillParams::default()
            },
            ..GalvoSettings::default()
        });
        let mut fill = Project::new();
        fill.cut_settings.push(fill_setting);
        fill.shapes.push(Shape {
            kind: ShapeKind::Rect,
            cut_index: 0,
            geometry: square(),
        });
        let fill_cmds = plan_galvo_project(&fill, &param, 30_000);

        // A 20 mm square hatched at 1 mm is about twenty scan lines, far more than an outline.
        assert!(
            fill_cmds.len() > outline_cmds.len() * 3,
            "fill {} outline {}",
            fill_cmds.len(),
            outline_cmds.len()
        );
    }

    #[test]
    fn galvo_marks_an_enclosed_shape_first() {
        let param = GalvoParam {
            field_size_mm: 50.0,
            resolution_bits: 16,
        };
        let outer = Geometry::Polyline {
            points: vec![
                Point2 { x: -20.0, y: -20.0 },
                Point2 { x: 20.0, y: -20.0 },
                Point2 { x: 20.0, y: 20.0 },
                Point2 { x: -20.0, y: 20.0 },
            ],
            closed: true,
        };
        let inner = Geometry::Polyline {
            points: vec![
                Point2 { x: 5.0, y: 5.0 },
                Point2 { x: 10.0, y: 5.0 },
                Point2 { x: 10.0, y: 10.0 },
                Point2 { x: 5.0, y: 10.0 },
            ],
            closed: true,
        };
        let mut project = Project::new();
        project.cut_settings.push(cut());
        // Outer first in document order, so document order would mark it first.
        project.shapes.push(Shape {
            kind: ShapeKind::Rect,
            cut_index: 0,
            geometry: outer,
        });
        project.shapes.push(Shape {
            kind: ShapeKind::Rect,
            cut_index: 0,
            geometry: inner,
        });
        let commands = plan_galvo_project(&project, &param, 30_000);
        let first_jump = commands.iter().find_map(|c| match c {
            GalvoCommand::Jump { x, y } => Some((*x, *y)),
            _ => None,
        });
        // The first jump lands on the inner square's start, not the outer's.
        assert_eq!(first_jump, Some(param.logic_to_device(5.0, 5.0)));
    }

    #[test]
    fn concentric_galvo_insets_rings_beyond_the_outline() {
        use lasercraft_laser_doc::{FillParams, GalvoSettings};
        let param = GalvoParam {
            field_size_mm: 110.0,
            resolution_bits: 16,
        };
        let square = || Geometry::Polyline {
            points: vec![
                Point2 { x: -20.0, y: -20.0 },
                Point2 { x: 20.0, y: -20.0 },
                Point2 { x: 20.0, y: 20.0 },
                Point2 { x: -20.0, y: 20.0 },
            ],
            closed: true,
        };
        let mut outline = Project::new();
        outline.cut_settings.push(cut());
        outline.shapes.push(Shape {
            kind: ShapeKind::Rect,
            cut_index: 0,
            geometry: square(),
        });
        let outline_cmds = plan_galvo_project(&outline, &param, 30_000);

        let mut conc_setting = cut();
        conc_setting.galvo = Some(GalvoSettings {
            mode: FillMode::Concentric,
            fill: FillParams {
                line_interval_mm: 2.0,
                ..FillParams::default()
            },
            ..GalvoSettings::default()
        });
        let mut conc = Project::new();
        conc.cut_settings.push(conc_setting);
        conc.shapes.push(Shape {
            kind: ShapeKind::Rect,
            cut_index: 0,
            geometry: square(),
        });
        let conc_cmds = plan_galvo_project(&conc, &param, 30_000);

        // A 40 mm square inset by 2 mm adds several rings, each with its own marks.
        assert!(
            conc_cmds.len() > outline_cmds.len() * 3,
            "concentric {} outline {}",
            conc_cmds.len(),
            outline_cmds.len()
        );
    }

    #[test]
    fn line_fill_job_clamps_feed_and_scales_power() {
        let bbox = BBox {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 10.0,
            max_y: 10.0,
        };
        let motion = MotionConfig {
            max_speed_mm_s: 100.0,
            accel_mm_s2: 1000.0,
            accel_to_decel_mm_s2: 500.0,
        };
        let out = plan_line_fill(&bbox, &cut(), &motion, 5.0, &GCodeDialect::grbl());
        // 20 mm/s clamps under 100 and renders as 1200 mm/min; 80 percent is S800.
        assert!(out.contains("F1200 S800"));
        assert!(out.contains("G1"));
    }

    #[test]
    fn plan_project_cuts_a_closed_polyline() {
        let motion = MotionConfig {
            max_speed_mm_s: 100.0,
            accel_mm_s2: 1000.0,
            accel_to_decel_mm_s2: 500.0,
        };
        let project = Project {
            app_version: "0.1.0".to_string(),
            shapes: vec![Shape {
                kind: ShapeKind::Path,
                cut_index: 0,
                geometry: Geometry::Polyline {
                    points: vec![
                        Point2 { x: 0.0, y: 0.0 },
                        Point2 { x: 10.0, y: 0.0 },
                        Point2 { x: 10.0, y: 10.0 },
                    ],
                    closed: true,
                },
            }],
            cut_settings: vec![cut()],
        };
        let out = plan_project(&project, &motion, &GCodeDialect::grbl());
        // Three edges of the closed triangle become three cutting moves.
        let cut_moves = out.lines().filter(|l| l.starts_with("G1 ")).count();
        assert_eq!(cut_moves, 3);
        // The cut runs at the layer feed and power.
        assert!(out.contains("F1200 S800"));
    }

    #[test]
    fn overscan_runs_fill_lines_past_the_edge_with_beam_off() {
        use lasercraft_laser_doc::{FillParams, GalvoSettings};
        let motion = MotionConfig {
            max_speed_mm_s: 100.0,
            accel_mm_s2: 1000.0,
            accel_to_decel_mm_s2: 500.0,
        };
        let mut setting = cut();
        setting.galvo = Some(GalvoSettings {
            mode: FillMode::Fill,
            fill: FillParams {
                line_interval_mm: 2.0,
                overscan_mm: 5.0,
                ..FillParams::default()
            },
            ..GalvoSettings::default()
        });
        let project = Project {
            app_version: "0.1.0".to_string(),
            shapes: vec![Shape {
                kind: ShapeKind::Rect,
                cut_index: 0,
                geometry: Geometry::Polyline {
                    points: vec![
                        Point2 { x: 0.0, y: 0.0 },
                        Point2 { x: 10.0, y: 0.0 },
                        Point2 { x: 10.0, y: 10.0 },
                        Point2 { x: 0.0, y: 10.0 },
                    ],
                    closed: true,
                },
            }],
            cut_settings: vec![setting],
        };
        let out = plan_project(&project, &motion, &GCodeDialect::grbl());
        // Overscan emits laser-off feed moves (S0) to approach and exit each line.
        assert!(
            out.lines()
                .any(|l| l.starts_with("G1 ") && l.ends_with("S0")),
            "expected an S0 feed move"
        );
        // A move reaches past the 10 mm right edge by the 5 mm overscan.
        let max_x = out
            .lines()
            .filter(|l| l.starts_with("G1 ") || l.starts_with("G0 "))
            .filter_map(|l| {
                l.split_whitespace()
                    .find_map(|w| w.strip_prefix('X').and_then(|v| v.parse::<f64>().ok()))
            })
            .fold(f64::MIN, f64::max);
        assert!(max_x > 14.0, "max_x {max_x}");
    }

    #[test]
    fn estimate_job_time_sums_trapezoidal_moves() {
        let motion = MotionConfig {
            max_speed_mm_s: 300.0,
            accel_mm_s2: 1000.0,
            accel_to_decel_mm_s2: 500.0,
        };
        // A closed square of four 20 mm edges at 25 mm/s.
        let project = Project {
            app_version: "0.1.0".to_string(),
            shapes: vec![Shape {
                kind: ShapeKind::Path,
                cut_index: 0,
                geometry: Geometry::Polyline {
                    points: vec![
                        Point2 { x: 0.0, y: 0.0 },
                        Point2 { x: 20.0, y: 0.0 },
                        Point2 { x: 20.0, y: 20.0 },
                        Point2 { x: 0.0, y: 20.0 },
                    ],
                    closed: true,
                },
            }],
            cut_settings: vec![CutSetting {
                index: 0,
                name: "Cut".to_string(),
                speed_mm_s: 25.0,
                power_min: 0.0,
                power_max: 80.0,
                num_passes: 1,
                air_assist: true,
                galvo: None,
            }],
        };
        let seconds = estimate_job_time(&project, &motion);
        // Each 20 mm edge at 25 mm/s with accel 1000 takes 0.825 s; four edges is 3.3 s.
        assert!((seconds - 3.3).abs() < 1e-6, "got {seconds}");
    }

    #[test]
    fn plan_frame_traces_the_job_bounds() {
        let project = Project {
            app_version: "0.1.0".to_string(),
            shapes: vec![Shape {
                kind: ShapeKind::Path,
                cut_index: 0,
                geometry: Geometry::Polyline {
                    points: vec![
                        Point2 { x: 2.0, y: 3.0 },
                        Point2 { x: 20.0, y: 3.0 },
                        Point2 { x: 11.0, y: 18.0 },
                    ],
                    closed: true,
                },
            }],
            cut_settings: vec![cut()],
        };
        let out = plan_frame(&project, &GCodeDialect::grbl());
        // Five rapids trace the bounding rectangle (2,3)-(20,18) and return.
        let rapids = out.lines().filter(|l| l.starts_with("G0 ")).count();
        // Four corners, the closing corner, plus GRBL's footer homing rapid.
        assert_eq!(rapids, 6);
        assert!(out.contains("G0 X2.000 Y3.000"));
        assert!(out.contains("G0 X20.000 Y18.000"));
        // Framing never fires the laser.
        assert!(!out.contains("G1 "));
    }

    #[test]
    fn project_to_svg_renders_shapes() {
        let project = Project {
            app_version: "0.1.0".to_string(),
            shapes: vec![Shape {
                kind: ShapeKind::Rect,
                cut_index: 0,
                geometry: Geometry::Polyline {
                    points: vec![
                        Point2 { x: 0.0, y: 0.0 },
                        Point2 { x: 20.0, y: 0.0 },
                        Point2 { x: 20.0, y: 20.0 },
                        Point2 { x: 0.0, y: 20.0 },
                    ],
                    closed: true,
                },
            }],
            cut_settings: vec![cut()],
        };
        let svg = project_to_svg(&project);
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("viewBox"));
        assert!(svg.contains("<path"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn material_test_builds_a_speed_power_grid() {
        let spec = MaterialTestSpec {
            columns: 3,
            rows: 2,
            cell_mm: 8.0,
            gap_mm: 2.0,
            speed_min: 10.0,
            speed_max: 30.0,
            power_min: 20.0,
            power_max: 80.0,
        };
        let project = material_test(&spec);
        // 3x2 grid is six squares, each with its own cut setting.
        assert_eq!(project.shapes.len(), 6);
        assert_eq!(project.cut_settings.len(), 6);
        // First cell is the slowest, lowest power; last cell the fastest, highest.
        assert_eq!(project.cut_settings[0].speed_mm_s, 10.0);
        assert_eq!(project.cut_settings[0].power_max, 20.0);
        let last = project.cut_settings.len() - 1;
        assert_eq!(project.cut_settings[last].speed_mm_s, 30.0);
        assert_eq!(project.cut_settings[last].power_max, 80.0);
    }

    #[test]
    fn optimize_order_visits_nearest_first() {
        let starts = [
            Point { x: 100.0, y: 0.0 },
            Point { x: 1.0, y: 0.0 },
            Point { x: 50.0, y: 0.0 },
        ];
        // From the origin: nearest is index 1, then 2, then 0.
        assert_eq!(optimize_order(&starts), vec![1, 2, 0]);
    }

    #[test]
    fn nested_order_cuts_inner_before_outer() {
        let outer = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 20.0, y: 0.0 },
            Point { x: 20.0, y: 20.0 },
            Point { x: 0.0, y: 20.0 },
        ];
        let inner = vec![
            Point { x: 8.0, y: 8.0 },
            Point { x: 12.0, y: 8.0 },
            Point { x: 12.0, y: 12.0 },
            Point { x: 8.0, y: 12.0 },
        ];
        // Run 0 is the outer square and sits nearer the origin, so a plain
        // nearest-neighbour walk would cut it first and free the inner piece.
        let starts = vec![outer[0], inner[0]];
        let points = vec![outer, inner];
        let closed = vec![true, true];
        let order = nested_order(&points, &closed, &starts);
        let pos_inner = order.iter().position(|&i| i == 1).unwrap_or(usize::MAX);
        let pos_outer = order.iter().position(|&i| i == 0).unwrap_or(usize::MAX);
        assert!(pos_inner < pos_outer, "order {order:?}");
    }

    #[test]
    fn plan_project_repeats_passes() {
        let motion = MotionConfig {
            max_speed_mm_s: 100.0,
            accel_mm_s2: 1000.0,
            accel_to_decel_mm_s2: 500.0,
        };
        let mut setting = cut();
        setting.num_passes = 2;
        let project = Project {
            app_version: "0.1.0".to_string(),
            shapes: vec![Shape {
                kind: ShapeKind::Path,
                cut_index: 0,
                geometry: Geometry::Polyline {
                    points: vec![
                        Point2 { x: 0.0, y: 0.0 },
                        Point2 { x: 10.0, y: 0.0 },
                        Point2 { x: 10.0, y: 10.0 },
                    ],
                    closed: true,
                },
            }],
            cut_settings: vec![setting],
        };
        let out = plan_project(&project, &motion, &GCodeDialect::grbl());
        // Three edges cut twice is six moves.
        let cut_moves = out.lines().filter(|l| l.starts_with("G1 ")).count();
        assert_eq!(cut_moves, 6);
    }

    #[test]
    fn plan_galvo_project_marks_a_polyline() {
        let param = GalvoParam {
            field_size_mm: 100.0,
            resolution_bits: 16,
        };
        let project = Project {
            app_version: "0.1.0".to_string(),
            shapes: vec![Shape {
                kind: ShapeKind::Path,
                cut_index: 0,
                geometry: Geometry::Polyline {
                    points: vec![
                        Point2 { x: 0.0, y: 0.0 },
                        Point2 { x: 10.0, y: 0.0 },
                        Point2 { x: 5.0, y: 8.0 },
                    ],
                    closed: true,
                },
            }],
            cut_settings: vec![cut()],
        };
        let commands = plan_galvo_project(&project, &param, 20000);
        let marks = commands
            .iter()
            .filter(|c| matches!(c, GalvoCommand::Mark { .. }))
            .count();
        // Two edges plus the closing edge.
        assert_eq!(marks, 3);
    }

    #[test]
    fn plan_raster_engraves_dark_pixels() {
        let motion = MotionConfig {
            max_speed_mm_s: 300.0,
            accel_mm_s2: 1000.0,
            accel_to_decel_mm_s2: 500.0,
        };
        let image = GrayImage {
            width: 3,
            height: 1,
            pixels: vec![0, 255, 255],
        };
        let params = RasterParams {
            pixel_mm: 1.0,
            power_max: 100.0,
            threshold: 128,
            speed_mm_s: 200.0,
        };
        let out = plan_raster(&image, &params, &motion, &GCodeDialect::grbl());
        // The two dark pixels merge into one engrave move at full power.
        let cut_moves = out.lines().filter(|l| l.starts_with("G1 ")).count();
        assert_eq!(cut_moves, 1);
        // 200 mm/s is 12000 mm/min, full power is S1000 on GRBL.
        assert!(out.contains("F12000 S1000"));
    }
}
