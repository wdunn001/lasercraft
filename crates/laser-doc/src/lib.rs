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

/// The concrete geometry a shape carries.
///
/// Polylines come first because every vector shape reduces to one once curves are
/// flattened. Bezier and primitive forms land on top of this later.
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
            }],
        };
        let shape = &project.shapes[0];
        let setting = project.cut_setting_for(shape);
        assert!(setting.is_some());
        assert_eq!(setting.map(|s| s.index), Some(2));
    }
}
