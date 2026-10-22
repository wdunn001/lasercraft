#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! Project persistence for LaserCraft.
//!
//! This is the first integration of the shared `craft-libs` workspace. It saves a
//! [`Project`] as JSON through `craft-atomic`, the suite's crash-safe atomic write,
//! so a power loss mid-save never leaves a half-written project on disk. The
//! native document format is JSON for now. A LightBurn `.lbrn` XML importer maps
//! onto the same [`Project`] model later.

use lasercraft_laser_doc::Project;
use std::io;
use std::path::Path;

/// Save a project to `path` as pretty JSON, replacing it atomically.
pub fn save_project(path: &Path, project: &Project) -> io::Result<()> {
    let bytes = serde_json::to_vec_pretty(project).map_err(io::Error::other)?;
    craft_atomic::atomic_write(path, &bytes)
}

/// Load a project from `path`.
pub fn load_project(path: &Path) -> io::Result<Project> {
    let bytes = std::fs::read(path)?;
    serde_json::from_slice(&bytes).map_err(io::Error::other)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lasercraft_laser_doc::{CutSetting, Geometry, Shape, ShapeKind};

    #[test]
    fn project_round_trips_through_an_atomic_save() -> io::Result<()> {
        let project = Project {
            app_version: "0.1.0".to_string(),
            shapes: vec![Shape {
                kind: ShapeKind::Rect,
                cut_index: 0,
                geometry: Geometry::Polyline {
                    points: vec![
                        lasercraft_laser_doc::Point2 { x: 0.0, y: 0.0 },
                        lasercraft_laser_doc::Point2 { x: 5.0, y: 0.0 },
                        lasercraft_laser_doc::Point2 { x: 5.0, y: 5.0 },
                        lasercraft_laser_doc::Point2 { x: 0.0, y: 5.0 },
                    ],
                    closed: true,
                },
            }],
            cut_settings: vec![CutSetting {
                index: 0,
                name: "Engrave".to_string(),
                speed_mm_s: 150.0,
                power_min: 5.0,
                power_max: 35.0,
                num_passes: 1,
                air_assist: false,
                galvo: None,
            }],
        };
        let path = std::env::temp_dir().join("lasercraft_persist_round_trip.json");
        save_project(&path, &project)?;
        let loaded = load_project(&path)?;
        assert_eq!(loaded, project);
        let _ = std::fs::remove_file(&path);
        Ok(())
    }
}
