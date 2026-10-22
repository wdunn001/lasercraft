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
use lasercraft_laser_doc::{CutSetting, Project, Shape};
use lasercraft_motion::MotionConfig;
use lasercraft_toolpath::{BBox, line_fill};

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
    let mut ops = Vec::with_capacity(segments.len() * 2);
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
    program(dialect, &ops)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lasercraft_gcode::Grbl;
    use lasercraft_laser_doc::ShapeKind;

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
}
