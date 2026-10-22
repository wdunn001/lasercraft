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
            }],
        };
        let shape = &project.shapes[0];
        let setting = project.cut_setting_for(shape);
        assert!(setting.is_some());
        assert_eq!(setting.map(|s| s.index), Some(2));
    }
}
