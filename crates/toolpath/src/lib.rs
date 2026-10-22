#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! Toolpath generation for LaserCraft.
//!
//! Reverse engineering of `acclplanner.dll` found the fill modes LightBurn uses:
//! line fill, dot fill, offset fill, and vector (`fill_galvo_line`,
//! `fill_galvo_dot`, `offset_galvo`, `vector_galvo`). See `docs/rea-findings.md`.
//! This crate turns geometry plus a fill mode into ordered moves. It starts with
//! the line fill, which is the workhorse for engraving a filled region.

/// A point in millimetres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    /// X in millimetres.
    pub x: f64,
    /// Y in millimetres.
    pub y: f64,
}

/// An axis-aligned bounding box in millimetres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BBox {
    /// Minimum X.
    pub min_x: f64,
    /// Minimum Y.
    pub min_y: f64,
    /// Maximum X.
    pub max_x: f64,
    /// Maximum Y.
    pub max_y: f64,
}

/// The supported fill modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillMode {
    /// Parallel scan lines.
    Line,
    /// A grid of dots.
    Dot,
    /// Contour-parallel offset passes.
    Offset,
    /// Follow the vector outline only.
    Vector,
}

/// A single straight cutting move from `start` to `end`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    /// Where the move starts.
    pub start: Point,
    /// Where the move ends.
    pub end: Point,
}

/// Generate horizontal line-fill scan segments across `bbox`.
///
/// Lines run left to right at `interval_mm` spacing from the bottom up. Alternate
/// lines reverse direction so the head does not fly back, the standard boustrophedon
/// engraving order. An interval at or below zero yields no segments.
pub fn line_fill(bbox: &BBox, interval_mm: f64) -> Vec<Segment> {
    let mut segments = Vec::new();
    if interval_mm <= 0.0 || bbox.max_y < bbox.min_y || bbox.max_x < bbox.min_x {
        return segments;
    }
    let mut y = bbox.min_y;
    let mut leftward = false;
    // Guard against a runaway loop from a tiny interval over a huge span.
    let max_lines = 1_000_000usize;
    while y <= bbox.max_y + f64::EPSILON && segments.len() < max_lines {
        let (x0, x1) = if leftward {
            (bbox.max_x, bbox.min_x)
        } else {
            (bbox.min_x, bbox.max_x)
        };
        segments.push(Segment {
            start: Point { x: x0, y },
            end: Point { x: x1, y },
        });
        leftward = !leftward;
        y += interval_mm;
    }
    segments
}

/// Trace a polyline as consecutive cutting segments.
///
/// Each pair of adjacent points becomes one segment. A `closed` polyline adds a
/// final segment from the last point back to the first. A polyline with fewer
/// than two points yields nothing.
pub fn follow_polyline(points: &[Point], closed: bool) -> Vec<Segment> {
    let mut segments = Vec::new();
    if points.len() < 2 {
        return segments;
    }
    for pair in points.windows(2) {
        segments.push(Segment {
            start: pair[0],
            end: pair[1],
        });
    }
    if closed {
        match (points.last(), points.first()) {
            (Some(last), Some(first)) if last != first => segments.push(Segment {
                start: *last,
                end: *first,
            }),
            _ => {}
        }
    }
    segments
}

/// Expand a polyline into a wobbled path: overlapping circles traced along it.
///
/// This is LightBurn's "Wobble" and EZCAD's "Dithering". The tool advances by
/// `step_mm` along the path and traces a circle of diameter `size_mm` at each
/// step, which widens the effective line for cutting a thin kerf. The result is a
/// dense polyline the caller then marks or cuts. A degenerate input returns
/// unchanged: fewer than two points, a non-positive step, or a non-positive size.
pub fn wobble_path(points: &[Point], step_mm: f64, size_mm: f64) -> Vec<Point> {
    const CIRCLE_POINTS: usize = 16;
    let radius = size_mm / 2.0;
    if points.len() < 2 || step_mm <= 0.0 || radius <= 0.0 {
        return points.to_vec();
    }
    let circle = |center: Point| {
        (0..=CIRCLE_POINTS).map(move |k| {
            let theta = std::f64::consts::TAU * (k as f64) / (CIRCLE_POINTS as f64);
            Point {
                x: center.x + radius * theta.cos(),
                y: center.y + radius * theta.sin(),
            }
        })
    };
    let mut out: Vec<Point> = Vec::new();
    let mut target = 0.0_f64;
    let mut travelled = 0.0_f64;
    for pair in points.windows(2) {
        let [a, b] = pair else { continue };
        let seg_len = ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt();
        if seg_len <= 0.0 {
            continue;
        }
        // Emit a circle each time a step target falls inside this segment.
        while target <= travelled + seg_len {
            let t = (target - travelled) / seg_len;
            let center = Point {
                x: a.x + (b.x - a.x) * t,
                y: a.y + (b.y - a.y) * t,
            };
            out.extend(circle(center));
            target += step_mm;
        }
        travelled += seg_len;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wobble_traces_circles_along_a_line() {
        let line = [Point { x: 0.0, y: 0.0 }, Point { x: 10.0, y: 0.0 }];
        let path = wobble_path(&line, 1.0, 0.5);
        // Roughly eleven circles (arc lengths 0 through 10 at step 1), each dense.
        assert!(path.len() > 100, "got {}", path.len());
        // Every point stays within the half-diameter band around the line.
        let radius = 0.25;
        for p in &path {
            assert!(
                p.x >= -radius - 1e-9 && p.x <= 10.0 + radius + 1e-9,
                "x {}",
                p.x
            );
            assert!(p.y.abs() <= radius + 1e-9, "y {}", p.y);
        }
    }

    #[test]
    fn wobble_leaves_a_degenerate_input_unchanged() {
        let one = [Point { x: 1.0, y: 2.0 }];
        assert_eq!(wobble_path(&one, 1.0, 0.5).len(), 1);
        let line = [Point { x: 0.0, y: 0.0 }, Point { x: 10.0, y: 0.0 }];
        assert_eq!(wobble_path(&line, 0.0, 0.5).len(), 2);
        assert_eq!(wobble_path(&line, 1.0, 0.0).len(), 2);
    }

    fn unit_box() -> BBox {
        BBox {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 10.0,
            max_y: 10.0,
        }
    }

    #[test]
    fn line_fill_counts_and_alternates() {
        let segments = line_fill(&unit_box(), 2.0);
        // y at 0,2,4,6,8,10 is six lines.
        assert_eq!(segments.len(), 6);
        // First line runs left to right, second reverses.
        assert_eq!(segments[0].start.x, 0.0);
        assert_eq!(segments[1].start.x, 10.0);
    }

    #[test]
    fn non_positive_interval_yields_nothing() {
        assert!(line_fill(&unit_box(), 0.0).is_empty());
        assert!(line_fill(&unit_box(), -1.0).is_empty());
    }

    #[test]
    fn follow_closed_triangle_makes_three_segments() {
        let points = [
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 5.0, y: 8.0 },
        ];
        let open = follow_polyline(&points, false);
        assert_eq!(open.len(), 2);
        let closed = follow_polyline(&points, true);
        assert_eq!(closed.len(), 3);
        // The closing segment returns to the first point.
        assert_eq!(closed[2].end, points[0]);
    }

    #[test]
    fn follow_degenerate_polyline_yields_nothing() {
        assert!(follow_polyline(&[Point { x: 1.0, y: 1.0 }], true).is_empty());
        assert!(follow_polyline(&[], false).is_empty());
    }
}
