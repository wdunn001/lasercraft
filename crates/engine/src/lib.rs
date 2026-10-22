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

use lasercraft_gcode::{Dialect, Op, program};
use lasercraft_laser_doc::{CutSetting, Geometry, Project, Shape};
use lasercraft_motion::MotionConfig;
use lasercraft_raster::{GrayImage, RasterParams, raster_scan};
use lasercraft_toolpath::{BBox, Point, follow_polyline, line_fill};

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
    /// Add a cut setting to the job.
    AddCutSetting(CutSetting),
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
            Command::AddCutSetting(setting) => self.project.cut_settings.push(setting),
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
    let mut ops = Vec::new();
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
        let segments = follow_polyline(&path, closed);
        let feed = motion.clamp_feed(setting.speed_mm_s);
        // Repeat the path once per configured pass. A zero-pass layer is off.
        for _ in 0..setting.num_passes {
            if let Some(first) = segments.first() {
                ops.push(Op::RapidTo {
                    x: first.start.x,
                    y: first.start.y,
                });
            }
            for segment in &segments {
                ops.push(Op::CutTo {
                    x: segment.end.x,
                    y: segment.end.y,
                    feed_mm_s: feed,
                    power: setting.power_max,
                });
            }
        }
    }
    program(dialect, &ops)
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
    use lasercraft_gcode::Grbl;
    use lasercraft_laser_doc::{Point2, ShapeKind};

    fn cut() -> CutSetting {
        CutSetting {
            index: 0,
            name: "Cut".to_string(),
            speed_mm_s: 20.0,
            power_min: 10.0,
            power_max: 80.0,
            num_passes: 1,
            air_assist: true,
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
        assert!(engine.execute(Command::AddCutSetting(cut())).is_ok());
        assert!(engine.execute(Command::AddShape(shape)).is_ok());
        assert_eq!(engine.project().shapes.len(), 1);
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
        let out = plan_line_fill(&bbox, &cut(), &motion, 5.0, &Grbl::new());
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
        let out = plan_project(&project, &motion, &Grbl::new());
        // Three edges of the closed triangle become three cutting moves.
        let cut_moves = out.lines().filter(|l| l.starts_with("G1 ")).count();
        assert_eq!(cut_moves, 3);
        // The cut runs at the layer feed and power.
        assert!(out.contains("F1200 S800"));
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
        let out = plan_frame(&project, &Grbl::new());
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
        let out = plan_project(&project, &motion, &Grbl::new());
        // Three edges cut twice is six moves.
        let cut_moves = out.lines().filter(|l| l.starts_with("G1 ")).count();
        assert_eq!(cut_moves, 6);
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
        let out = plan_raster(&image, &params, &motion, &Grbl::new());
        // The two dark pixels merge into one engrave move at full power.
        let cut_moves = out.lines().filter(|l| l.starts_with("G1 ")).count();
        assert_eq!(cut_moves, 1);
        // 200 mm/s is 12000 mm/min, full power is S1000 on GRBL.
        assert!(out.contains("F12000 S1000"));
    }
}
