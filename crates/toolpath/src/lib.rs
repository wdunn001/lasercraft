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

/// Generate a grid of dot positions across `bbox` at `interval_mm` spacing.
///
/// This is LightBurn's dot fill. Each dot is a position the caller pulses the
/// laser at, row by row from the bottom left. A non-positive interval, or a box
/// with no area, yields nothing. The count is capped so a tiny interval over a
/// huge box cannot run away.
pub fn dot_fill(bbox: &BBox, interval_mm: f64) -> Vec<Point> {
    let mut dots = Vec::new();
    if interval_mm <= 0.0 || bbox.max_x < bbox.min_x || bbox.max_y < bbox.min_y {
        return dots;
    }
    let max_dots = 4_000_000usize;
    let mut y = bbox.min_y;
    while y <= bbox.max_y + f64::EPSILON && dots.len() < max_dots {
        let mut x = bbox.min_x;
        while x <= bbox.max_x + f64::EPSILON && dots.len() < max_dots {
            dots.push(Point { x, y });
            x += interval_mm;
        }
        y += interval_mm;
    }
    dots
}

/// Generate contour-parallel offset passes inward from `bbox`.
///
/// This is LightBurn's offset fill for a rectangular region. Each ring insets by
/// `interval_mm` from the one outside it and is four segments. Rings stop when the
/// next would cross the centre. A non-positive interval yields nothing. Arbitrary
/// outlines need a polygon offset, which the shared path-operations crate carries.
pub fn offset_fill(bbox: &BBox, interval_mm: f64) -> Vec<Segment> {
    let mut segments = Vec::new();
    if interval_mm <= 0.0 {
        return segments;
    }
    let max_rings = 1_000_000usize;
    let mut inset = 0.0;
    let mut rings = 0usize;
    loop {
        let (x0, y0) = (bbox.min_x + inset, bbox.min_y + inset);
        let (x1, y1) = (bbox.max_x - inset, bbox.max_y - inset);
        if x1 <= x0 || y1 <= y0 || rings >= max_rings {
            break;
        }
        let corners = [
            Point { x: x0, y: y0 },
            Point { x: x1, y: y0 },
            Point { x: x1, y: y1 },
            Point { x: x0, y: y1 },
            Point { x: x0, y: y0 },
        ];
        for pair in corners.windows(2) {
            let [a, b] = pair else { continue };
            segments.push(Segment { start: *a, end: *b });
        }
        inset += interval_mm;
        rings += 1;
    }
    segments
}

/// Fill the interior of a closed polygon with horizontal scan lines.
///
/// This is the true line fill for an arbitrary outline, clipped to the shape
/// rather than its bounding box. Each scan line crosses the polygon at an even
/// number of edges, and the laser marks between consecutive pairs, the even-odd
/// rule. Alternate lines reverse so the head does not fly back. A polygon with
/// fewer than three points, or a non-positive interval, yields nothing. The line
/// count is capped against a runaway.
pub fn hatch_fill(polygon: &[Point], interval_mm: f64) -> Vec<Segment> {
    let mut segments = Vec::new();
    if polygon.len() < 3 || interval_mm <= 0.0 {
        return segments;
    }
    let (min_y, max_y) = polygon
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| {
            (lo.min(p.y), hi.max(p.y))
        });
    let n = polygon.len();
    let max_lines = 1_000_000usize;
    let mut y = min_y + interval_mm / 2.0;
    let mut leftward = false;
    while y <= max_y && segments.len() < max_lines {
        // The x where each crossing edge meets this scan line.
        let mut crossings: Vec<f64> = Vec::new();
        for i in 0..n {
            let (Some(&a), Some(&b)) = (polygon.get(i), polygon.get((i + 1) % n)) else {
                continue;
            };
            // A half-open test counts a vertex once, so a shared vertex is not doubled.
            let crosses = (a.y <= y && b.y > y) || (b.y <= y && a.y > y);
            if crosses {
                let t = (y - a.y) / (b.y - a.y);
                crossings.push(a.x + t * (b.x - a.x));
            }
        }
        crossings.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mut spans: Vec<(f64, f64)> = crossings
            .chunks(2)
            .filter_map(|pair| {
                if let [x0, x1] = pair {
                    Some((*x0, *x1))
                } else {
                    None
                }
            })
            .collect();
        if leftward {
            spans.reverse();
        }
        for (x0, x1) in spans {
            let (start_x, end_x) = if leftward { (x1, x0) } else { (x0, x1) };
            segments.push(Segment {
                start: Point { x: start_x, y },
                end: Point { x: end_x, y },
            });
        }
        leftward = !leftward;
        y += interval_mm;
    }
    segments
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

    #[test]
    fn dot_fill_makes_a_grid() {
        let dots = dot_fill(&unit_box(), 2.0);
        // x and y each at 0, 2, 4, 6, 8, 10 is six positions, so thirty-six dots.
        assert_eq!(dots.len(), 36);
    }

    #[test]
    fn dot_fill_rejects_a_non_positive_interval() {
        assert!(dot_fill(&unit_box(), 0.0).is_empty());
    }

    #[test]
    fn offset_fill_rings_inward() {
        let segments = offset_fill(&unit_box(), 2.0);
        // Rings at inset 0, 2 and 4 before the centre is crossed, four segments each.
        assert_eq!(segments.len(), 12);
    }

    #[test]
    fn offset_fill_rejects_a_non_positive_interval() {
        assert!(offset_fill(&unit_box(), 0.0).is_empty());
    }

    #[test]
    fn hatch_fill_scans_a_square() {
        let square = [
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
            Point { x: 0.0, y: 10.0 },
        ];
        let lines = hatch_fill(&square, 1.0);
        // Scan lines centred at y = 0.5 through 9.5 is ten lines.
        assert_eq!(lines.len(), 10);
        for seg in &lines {
            let lo = seg.start.x.min(seg.end.x);
            let hi = seg.start.x.max(seg.end.x);
            assert!(
                (lo - 0.0).abs() < 1e-9 && (hi - 10.0).abs() < 1e-9,
                "span {lo}..{hi}"
            );
        }
    }

    #[test]
    fn hatch_fill_narrows_over_a_triangle() {
        let tri = [
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 5.0, y: 10.0 },
        ];
        let lines = hatch_fill(&tri, 2.0);
        assert!(!lines.is_empty());
        let width = |s: &Segment| (s.start.x - s.end.x).abs();
        let first = lines.first().map(width).unwrap_or(0.0);
        let last = lines.last().map(width).unwrap_or(0.0);
        // A lower scan line crosses more of the triangle than a higher one.
        assert!(first > last, "first {first} last {last}");
    }

    #[test]
    fn hatch_fill_rejects_degenerate_input() {
        let two = [Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }];
        assert!(hatch_fill(&two, 1.0).is_empty());
        let square = [
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
            Point { x: 0.0, y: 10.0 },
        ];
        assert!(hatch_fill(&square, 0.0).is_empty());
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
