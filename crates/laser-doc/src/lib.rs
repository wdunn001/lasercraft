#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! The LaserCraft job document model.
//!
//! Reverse engineering showed the LightBurn project format (`.lbrn`, `.lbrn2`) is
//! plain XML with a `<LightBurnProject>` root and `<Shape>` elements carrying a
//! `Type` and a `CutIndex`. See `docs/rea-findings.md`. These types are the native
//! LaserCraft model. An importer for the LightBurn XML lands later and maps onto
//! them, which keeps the model independent of any one on-disk format.

use serde::{Deserialize, Serialize};

/// A complete job: the geometry plus the cut settings its shapes reference.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Project {
    /// The application version that wrote the project, informational only.
    pub app_version: String,
    /// The geometry in the job.
    pub shapes: Vec<Shape>,
    /// The cut settings, indexed by [`Shape::cut_index`].
    pub cut_settings: Vec<CutSetting>,
}

/// A point in millimetres in the work coordinate system.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point2 {
    /// X in millimetres.
    pub x: f64,
    /// Y in millimetres.
    pub y: f64,
}

/// One segment of a vector path, starting from the path's current point.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PathSeg {
    /// A straight line to `to`.
    Line {
        /// The segment end point.
        to: Point2,
    },
    /// A cubic bezier with control points `c1` and `c2` ending at `to`.
    Cubic {
        /// The first control point.
        c1: Point2,
        /// The second control point.
        c2: Point2,
        /// The segment end point.
        to: Point2,
    },
}

/// A vector path of line and cubic segments from a start point.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BezierPath {
    /// Where the path begins.
    pub start: Point2,
    /// The segments in order.
    pub segments: Vec<PathSeg>,
    /// Whether the path closes back to its start.
    pub closed: bool,
}

impl BezierPath {
    /// Flatten the path to a polyline, subdividing each cubic into `steps` chords.
    ///
    /// A cubic is sampled by the Bernstein form at evenly spaced `t`. Uniform
    /// sampling is simple and correct. Adaptive flattening by flatness tolerance is
    /// a later refinement.
    pub fn flatten(&self, steps: u32) -> Vec<Point2> {
        let mut points = vec![self.start];
        let mut current = self.start;
        let steps = steps.max(1);
        for segment in &self.segments {
            match segment {
                PathSeg::Line { to } => {
                    points.push(*to);
                    current = *to;
                }
                PathSeg::Cubic { c1, c2, to } => {
                    for i in 1..=steps {
                        let t = f64::from(i) / f64::from(steps);
                        points.push(cubic_point(current, *c1, *c2, *to, t));
                    }
                    current = *to;
                }
            }
        }
        points
    }
}

/// Evaluate a cubic bezier at parameter `t` using the Bernstein form.
fn cubic_point(p0: Point2, p1: Point2, p2: Point2, p3: Point2, t: f64) -> Point2 {
    let u = 1.0 - t;
    let w0 = u * u * u;
    let w1 = 3.0 * u * u * t;
    let w2 = 3.0 * u * t * t;
    let w3 = t * t * t;
    Point2 {
        x: w0 * p0.x + w1 * p1.x + w2 * p2.x + w3 * p3.x,
        y: w0 * p0.y + w1 * p1.y + w2 * p2.y + w3 * p3.y,
    }
}

/// The concrete geometry a shape carries.
///
/// Polylines are the flattened form every vector shape reduces to. A bezier path
/// carries curves and flattens to a polyline for toolpathing.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum Geometry {
    /// No geometry yet, for example an empty group.
    #[default]
    Empty,
    /// An ordered run of points, optionally closed back to the first.
    Polyline {
        /// The vertices in order.
        points: Vec<Point2>,
        /// Whether the last point connects back to the first.
        closed: bool,
    },
    /// A vector path of line and cubic segments.
    Path(BezierPath),
}

/// One piece of geometry. `cut_index` selects the [`CutSetting`] that drives it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shape {
    /// The kind of shape, mirroring the LightBurn `Type` attribute.
    pub kind: ShapeKind,
    /// Index into [`Project::cut_settings`], mirroring the `CutIndex` attribute.
    pub cut_index: u32,
    /// The concrete geometry. Defaults to [`Geometry::Empty`].
    #[serde(default)]
    pub geometry: Geometry,
}

/// The supported shape kinds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ShapeKind {
    /// A group of child shapes.
    Group,
    /// An open or closed path.
    Path,
    /// A rectangle.
    Rect,
    /// An ellipse.
    Ellipse,
    /// A text run.
    Text,
    /// A raster image.
    Image,
}

/// How a layer is cut: a reproduction of the LightBurn cut-setting fields.
///
/// Power is a percentage from 0.0 to 100.0. Speed is millimetres per second.
/// Air assist reflects the `airflow` config recovered from the binary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CutSetting {
    /// The cut index shapes point at.
    pub index: u32,
    /// The display name of the layer.
    pub name: String,
    /// Travel speed in millimetres per second.
    pub speed_mm_s: f64,
    /// Minimum power percentage, used at corners and low speed.
    pub power_min: f64,
    /// Maximum power percentage.
    pub power_max: f64,
    /// Number of passes over the geometry.
    pub num_passes: u32,
    /// Whether air assist runs for this layer.
    pub air_assist: bool,
    /// Galvo and fiber layer settings. Present when the machine is a galvo.
    /// A GCode machine leaves this `None`. Older projects without the field load
    /// as `None`.
    #[serde(default)]
    pub galvo: Option<GalvoSettings>,
}

/// How a layer fills or outlines its shapes. The names in the LaserCraft
/// interface are plain. The doc comments carry the LightBurn and EZCAD aliases.
/// See `docs/terminology-ezcad-lightburn.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FillMode {
    /// Trace the outline of each shape. LightBurn "Line".
    #[default]
    Line,
    /// Sweep back and forth to fill a region. LightBurn "Fill", EZCAD hatch type 2.
    Fill,
    /// Fill inward along offsets of the outline. LightBurn "Offset Fill", EZCAD type 3.
    Concentric,
    /// Engrave a raster image. LightBurn "Image".
    Image,
}

/// Fill shaping for the Fill and Concentric modes.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FillParams {
    /// Gap between fill lines in millimetres. LightBurn "Line Interval", EZCAD "Line Space".
    pub line_interval_mm: f64,
    /// Direction of the fill lines in degrees. LightBurn "Angle", EZCAD "Hatch Angle".
    pub hatch_angle_deg: f64,
    /// Angle added to each successive pass in degrees. LightBurn "Angle Increment".
    pub angle_increment_deg: f64,
    /// Advance the angle every time the job runs. LightBurn "Auto-Rotate".
    pub auto_rotate: bool,
    /// Shorten scan lines to offset beam width. LightBurn "Dot Width Adjust".
    pub dot_width_adjust: bool,
    /// The beam-width compensation in millimetres.
    pub dot_width_mm: f64,
    /// Taper the edge over this distance in millimetres. LightBurn "Ramp Length".
    pub ramp_length_mm: f64,
    /// Ramp the outer edge as well as the inner. LightBurn "Ramp Outer Edge".
    pub ramp_outer_edge: bool,
    /// Add a second fill pass perpendicular to the first. LightBurn "Cross-Hatch".
    #[serde(default)]
    pub cross_hatch: bool,
    /// Run each scan line past the fill edge by this distance with the beam off,
    /// so the head is at speed before it enters. LightBurn "Overscanning".
    #[serde(default)]
    pub overscan_mm: f64,
    /// Scan both directions, alternating line by line. Off scans one direction
    /// only, with a rapid return. LightBurn "Bi-directional scanning".
    #[serde(default = "default_true")]
    pub bidirectional: bool,
    /// Fill every shape on the layer as one field, so a scan line crosses several
    /// shapes at once. LightBurn "flood fill" / "fill all shapes at once".
    #[serde(default)]
    pub flood_fill: bool,
    /// Pulse the laser at a grid of dots instead of scanning lines. LightBurn
    /// "Dot Mode". The grid spacing is the line interval.
    #[serde(default)]
    pub dot_mode: bool,
    /// Dwell at each dot in milliseconds when dot mode is on.
    #[serde(default = "default_dot_time")]
    pub dot_time_ms: f64,
}

/// Serde default for the dot dwell time, a short pulse.
fn default_dot_time() -> f64 {
    1.0
}

/// Serde default for a flag that should load as `true` on older projects.
fn default_true() -> bool {
    true
}

impl Default for FillParams {
    fn default() -> Self {
        Self {
            line_interval_mm: 0.1,
            hatch_angle_deg: 0.0,
            angle_increment_deg: 0.0,
            auto_rotate: false,
            dot_width_adjust: false,
            dot_width_mm: 0.08,
            ramp_length_mm: 0.0,
            ramp_outer_edge: false,
            cross_hatch: false,
            overscan_mm: 0.0,
            bidirectional: true,
            flood_fill: false,
            dot_mode: false,
            dot_time_ms: 1.0,
        }
    }
}

/// Line shaping for the Line mode.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LineParams {
    /// Shift the cut to hold final size, in millimetres. LightBurn "Kerf offset".
    pub kerf_offset_mm: f64,
    /// Cut a dashed line when on. LightBurn "Perforation Mode".
    pub perforation: bool,
    /// The cut length of a perforation dash in millimetres.
    pub perforation_cut_mm: f64,
    /// The skipped gap of a perforation dash in millimetres.
    pub perforation_skip_mm: f64,
    /// Continue cutting past a closed outline's start, in millimetres, so
    /// controller lag does not leave the corner open. LightBurn "Overcut".
    #[serde(default)]
    pub overcut_mm: f64,
    /// Approach the outline start with a cut of this length, in millimetres, so
    /// the pierce sits off the final contour. LightBurn "Lead In".
    #[serde(default)]
    pub lead_in_mm: f64,
    /// Continue past the outline end with a cut of this length, in millimetres.
    /// LightBurn "Lead Out".
    #[serde(default)]
    pub lead_out_mm: f64,
}

impl Default for LineParams {
    fn default() -> Self {
        Self {
            kerf_offset_mm: 0.0,
            perforation: false,
            perforation_cut_mm: 0.1,
            perforation_skip_mm: 0.1,
            overcut_mm: 0.0,
            lead_in_mm: 0.0,
            lead_out_mm: 0.0,
        }
    }
}

/// Tabs and bridges: small uncut spans that hold the part in the sheet.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Tabs {
    /// Whether tabs are generated for this layer. LightBurn "Tabs / Bridges".
    pub enabled: bool,
    /// Place tabs automatically. The alternative is manual placement.
    pub automatic: bool,
    /// The length of each tab in millimetres. LightBurn "Tab Size".
    pub size_mm: f64,
    /// The distance between tabs in millimetres. LightBurn "Spacing".
    pub spacing_mm: f64,
    /// The number of tabs on each shape. LightBurn "Tabs Per Shape".
    pub tabs_per_shape: u32,
    /// Leave the inner shapes untabbed. LightBurn "Skip Inner Shapes".
    pub skip_inner_shapes: bool,
    /// The power at a tab as a percentage of max power. LightBurn "Tab Cut Power".
    pub cut_power_pct: f64,
}

impl Default for Tabs {
    fn default() -> Self {
        Self {
            enabled: false,
            automatic: true,
            size_mm: 0.5,
            spacing_mm: 50.0,
            tabs_per_shape: 1,
            skip_inner_shapes: false,
            cut_power_pct: 0.0,
        }
    }
}

/// Wobble: trace small circles along the path to widen the line.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Wobble {
    /// Whether wobble is on. LightBurn "Wobble", EZCAD "Dithering".
    pub enabled: bool,
    /// The spacing of the circles along the path in millimetres.
    pub step_mm: f64,
    /// The diameter of the circles in millimetres.
    pub size_mm: f64,
}

impl Default for Wobble {
    fn default() -> Self {
        Self {
            enabled: false,
            step_mm: 0.05,
            size_mm: 0.1,
        }
    }
}

/// A per-layer override of the device galvo timings. LightBurn "Override Default
/// Timings". The fields mirror the device defaults in `lasercraft-profile`. TC is
/// a dwell in microseconds that compensates for the mirror lag.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CutTiming {
    /// Travel speed between marks in millimetres per second. LightBurn "Jump Speed".
    pub jump_speed_mm_s: f64,
    /// Settle time after a short jump in microseconds. LightBurn "Min Jump Delay".
    pub min_jump_delay_us: f64,
    /// Settle time after a long jump in microseconds. LightBurn "Max Jump Delay".
    pub max_jump_delay_us: f64,
    /// Distance above which the max delay applies, in millimetres. LightBurn "Jump Distance Limit".
    pub jump_distance_limit_mm: f64,
    /// Wait after the mirrors start before firing, in microseconds. LightBurn "Laser On TC".
    pub laser_on_tc_us: f64,
    /// Wait after a mark ends before the laser stops, in microseconds. LightBurn "Laser Off TC".
    pub laser_off_tc_us: f64,
    /// Wait at the end of a mark segment, in microseconds. LightBurn "End TC".
    pub end_tc_us: f64,
    /// Pause at a corner between two segments, in microseconds. LightBurn "Polygon TC".
    pub polygon_tc_us: f64,
    /// Wait before the sub-layer starts, in milliseconds. LightBurn "Pause Before".
    pub pause_before_ms: f64,
    /// Wait after the sub-layer ends, in milliseconds. LightBurn "Pause After".
    pub pause_after_ms: f64,
}

impl Default for CutTiming {
    fn default() -> Self {
        Self {
            jump_speed_mm_s: 4000.0,
            min_jump_delay_us: 500.0,
            max_jump_delay_us: 500.0,
            jump_distance_limit_mm: 0.0,
            laser_on_tc_us: -300.0,
            laser_off_tc_us: 100.0,
            end_tc_us: 300.0,
            polygon_tc_us: 100.0,
            pause_before_ms: 0.0,
            pause_after_ms: 0.0,
        }
    }
}

/// Galvo and fiber layer settings. These back the Cut Settings editor's mode
/// selector, its shared parameter card, and its Cut and Timings tabs. See
/// `docs/cut-settings-design.md`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GalvoSettings {
    /// The fill or outline mode. The editor shows this as a segmented selector.
    pub mode: FillMode,
    /// Pulse repetition rate in kHz. LightBurn "Frequency".
    pub frequency_khz: f64,
    /// Pulse duration in nanoseconds, MOPA and UV only. LightBurn "Q-Pulse Width".
    pub q_pulse_width_ns: f64,
    /// Repeats of every sub-layer in sequence. LightBurn "Global Passes".
    pub global_passes: u32,
    /// Fill shaping, read by the Cut tab in Fill and Concentric modes.
    pub fill: FillParams,
    /// Line shaping, read by the Cut tab in Line mode.
    pub line: LineParams,
    /// Tabs and bridges, read by the Cut tab in Line mode.
    pub tabs: Tabs,
    /// Wobble, shared by Line and Fill.
    pub wobble: Wobble,
    /// A per-layer timing override. `None` uses the device defaults.
    pub timing_override: Option<CutTiming>,
}

impl Default for GalvoSettings {
    fn default() -> Self {
        Self {
            mode: FillMode::Line,
            frequency_khz: 30.0,
            q_pulse_width_ns: 200.0,
            global_passes: 1,
            fill: FillParams::default(),
            line: LineParams::default(),
            tabs: Tabs::default(),
            wobble: Wobble::default(),
            timing_override: None,
        }
    }
}

impl Project {
    /// Create an empty project.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resolve the cut setting a shape points at.
    pub fn cut_setting_for(&self, shape: &Shape) -> Option<&CutSetting> {
        self.cut_settings
            .iter()
            .find(|c| c.index == shape.cut_index)
    }

    /// The axis-aligned bounds of all geometry as `(min, max)`, or `None` when the
    /// project has no geometry.
    ///
    /// Bezier segments contribute their control points, which is a conservative
    /// superset of the true curve bounds. That is exactly what a framing pass wants,
    /// since it must enclose the whole job.
    pub fn bounds(&self) -> Option<(Point2, Point2)> {
        let mut points = self
            .shapes
            .iter()
            .flat_map(|s| geometry_points(&s.geometry));
        let first = points.next()?;
        let mut min = first;
        let mut max = first;
        for p in points {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
        }
        Some((min, max))
    }
}

/// Every point that contributes to a geometry's bounds.
fn geometry_points(geometry: &Geometry) -> Vec<Point2> {
    match geometry {
        Geometry::Empty => Vec::new(),
        Geometry::Polyline { points, .. } => points.clone(),
        Geometry::Path(path) => {
            let mut points = vec![path.start];
            for segment in &path.segments {
                match segment {
                    PathSeg::Line { to } => points.push(*to),
                    PathSeg::Cubic { c1, c2, to } => {
                        points.push(*c1);
                        points.push(*c2);
                        points.push(*to);
                    }
                }
            }
            points
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cubic_flattens_through_its_midpoint() {
        let path = BezierPath {
            start: Point2 { x: 0.0, y: 0.0 },
            segments: vec![PathSeg::Cubic {
                c1: Point2 { x: 0.0, y: 10.0 },
                c2: Point2 { x: 10.0, y: 10.0 },
                to: Point2 { x: 10.0, y: 0.0 },
            }],
            closed: false,
        };
        let points = path.flatten(2);
        // Start, midpoint, end for two steps.
        assert_eq!(points.len(), 3);
        assert_eq!(points[0], Point2 { x: 0.0, y: 0.0 });
        // The Bernstein midpoint of this symmetric cubic is (5, 7.5).
        assert!((points[1].x - 5.0).abs() < 1e-9);
        assert!((points[1].y - 7.5).abs() < 1e-9);
        assert_eq!(points[2], Point2 { x: 10.0, y: 0.0 });
    }

    #[test]
    fn shape_resolves_its_cut_setting() {
        let project = Project {
            app_version: "0.1.0".to_string(),
            shapes: vec![Shape {
                kind: ShapeKind::Path,
                cut_index: 2,
                geometry: Geometry::Polyline {
                    points: vec![
                        Point2 { x: 0.0, y: 0.0 },
                        Point2 { x: 10.0, y: 0.0 },
                        Point2 { x: 10.0, y: 10.0 },
                    ],
                    closed: true,
                },
            }],
            cut_settings: vec![CutSetting {
                index: 2,
                name: "Cut".to_string(),
                speed_mm_s: 20.0,
                power_min: 10.0,
                power_max: 80.0,
                num_passes: 1,
                air_assist: true,
                galvo: None,
            }],
        };
        let shape = &project.shapes[0];
        let setting = project.cut_setting_for(shape);
        assert!(setting.is_some());
        assert_eq!(setting.map(|s| s.index), Some(2));
    }
}
