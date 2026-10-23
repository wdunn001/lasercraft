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

/// Hatch fill at an arbitrary scan angle, with an optional perpendicular
/// cross-hatch pass. The polygon is rotated by `-angle` about its centroid so the
/// existing horizontal scan applies, then every segment is rotated back. A
/// `cross_hatch` pass adds a second set of lines at `angle + 90`.
pub fn hatch_fill_angled(
    polygon: &[Point],
    interval_mm: f64,
    angle_deg: f64,
    cross_hatch: bool,
) -> Vec<Segment> {
    if polygon.len() < 3 || interval_mm <= 0.0 {
        return Vec::new();
    }
    let (cx, cy) = {
        let (sx, sy) = polygon
            .iter()
            .fold((0.0, 0.0), |(ax, ay), p| (ax + p.x, ay + p.y));
        let k = polygon.len() as f64;
        (sx / k, sy / k)
    };
    let rot = |p: &Point, th: f64| {
        let (s, c) = th.sin_cos();
        let (dx, dy) = (p.x - cx, p.y - cy);
        Point {
            x: cx + dx * c - dy * s,
            y: cy + dx * s + dy * c,
        }
    };
    let one = |deg: f64| -> Vec<Segment> {
        let th = deg.to_radians();
        let turned: Vec<Point> = polygon.iter().map(|p| rot(p, -th)).collect();
        hatch_fill(&turned, interval_mm)
            .into_iter()
            .map(|s| Segment {
                start: rot(&s.start, th),
                end: rot(&s.end, th),
            })
            .collect()
    };
    let mut out = one(angle_deg);
    if cross_hatch {
        out.extend(one(angle_deg + 90.0));
    }
    out
}

/// Dash a traced outline for perforation (LightBurn "Perforation Mode").
///
/// The outline is walked as one continuous path and broken into `cut_mm` dashes
/// separated by `skip_mm` gaps. The dash phase carries across segment corners so
/// the pattern stays even around the shape. A non-positive cut or skip length
/// returns the solid outline unchanged.
pub fn perforate_path(points: &[Point], closed: bool, cut_mm: f64, skip_mm: f64) -> Vec<Segment> {
    let solid = follow_polyline(points, closed);
    if cut_mm <= 0.0 || skip_mm <= 0.0 {
        return solid;
    }
    let mut out = Vec::new();
    let mut cutting = true;
    let mut remaining = cut_mm;
    for seg in &solid {
        let (dx, dy) = (seg.end.x - seg.start.x, seg.end.y - seg.start.y);
        let len = dx.hypot(dy);
        if len <= f64::EPSILON {
            continue;
        }
        let (ux, uy) = (dx / len, dy / len);
        let mut pos = 0.0;
        let mut cur = seg.start;
        while pos < len - 1e-9 {
            let step = remaining.min(len - pos);
            let next = Point {
                x: cur.x + ux * step,
                y: cur.y + uy * step,
            };
            if cutting {
                out.push(Segment {
                    start: cur,
                    end: next,
                });
            }
            cur = next;
            pos += step;
            remaining -= step;
            if remaining <= 1e-9 {
                cutting = !cutting;
                remaining = if cutting { cut_mm } else { skip_mm };
            }
        }
    }
    out
}

/// Continue cutting past the start of a closed outline by `overcut_mm`
/// (LightBurn "Overcut"), so controller lag does not leave the start corner
/// open. Returns the extra segments to append after the outline is traced. An
/// open path or a non-positive distance yields nothing.
pub fn overcut_segments(points: &[Point], closed: bool, overcut_mm: f64) -> Vec<Segment> {
    if !closed || overcut_mm <= 0.0 || points.len() < 2 {
        return Vec::new();
    }
    let n = points.len();
    let mut out = Vec::new();
    let mut remaining = overcut_mm;
    let mut cur = points[0];
    // After closing, the head sits at points[0]; continue onto points[1], [2], ...
    let mut i = 1usize;
    let mut guard = 0usize;
    while remaining > 1e-9 && guard < 4 * n + 4 {
        guard += 1;
        let target = points[i % n];
        let (dx, dy) = (target.x - cur.x, target.y - cur.y);
        let len = dx.hypot(dy);
        if len <= f64::EPSILON {
            i += 1;
            continue;
        }
        let step = remaining.min(len);
        let (ux, uy) = (dx / len, dy / len);
        let next = Point {
            x: cur.x + ux * step,
            y: cur.y + uy * step,
        };
        out.push(Segment {
            start: cur,
            end: next,
        });
        cur = next;
        remaining -= step;
        if step >= len - 1e-9 {
            i += 1;
        }
    }
    out
}

/// A lead-in segment that approaches the outline start along the reverse of the
/// first edge's direction (LightBurn "Lead In", line type). It cuts from a point
/// `len_mm` away into the start vertex, so the pierce sits off the final contour.
/// Returns `None` for a degenerate path or a non-positive length.
pub fn lead_in_segment(points: &[Point], len_mm: f64) -> Option<Segment> {
    if len_mm <= 0.0 || points.len() < 2 {
        return None;
    }
    let (a, b) = (points[0], points[1]);
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    let d = dx.hypot(dy);
    if d <= f64::EPSILON {
        return None;
    }
    let (ux, uy) = (dx / d, dy / d);
    Some(Segment {
        start: Point {
            x: a.x - ux * len_mm,
            y: a.y - uy * len_mm,
        },
        end: a,
    })
}

/// A lead-out segment continuing past the outline end along the last edge's
/// direction (LightBurn "Lead Out"). For a closed path the cut ends at the first
/// vertex arriving along the closing edge, so the lead extends from there.
pub fn lead_out_segment(points: &[Point], closed: bool, len_mm: f64) -> Option<Segment> {
    if len_mm <= 0.0 || points.len() < 2 {
        return None;
    }
    let (a, b) = if closed {
        (*points.last()?, points[0])
    } else {
        (points[points.len() - 2], points[points.len() - 1])
    };
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    let d = dx.hypot(dy);
    if d <= f64::EPSILON {
        return None;
    }
    let (ux, uy) = (dx / d, dy / d);
    Some(Segment {
        start: b,
        end: Point {
            x: b.x + ux * len_mm,
            y: b.y + uy * len_mm,
        },
    })
}

/// Punch evenly spaced uncut gaps into a traced outline (LightBurn "Tabs /
/// Bridges"), leaving small bridges that hold the part in the sheet.
///
/// `count` gaps of `size_mm` are centred at equal arc-length intervals around
/// the whole outline. A non-positive count or size, an empty outline, or an
/// outline too short to hold the tabs with cut between them returns the segments
/// unchanged.
pub fn apply_tabs(segments: &[Segment], count: u32, size_mm: f64) -> Vec<Segment> {
    if count == 0 || size_mm <= 0.0 || segments.is_empty() {
        return segments.to_vec();
    }
    let total: f64 = segments
        .iter()
        .map(|s| (s.end.x - s.start.x).hypot(s.end.y - s.start.y))
        .sum();
    // Refuse to tab an outline the tabs would nearly consume.
    if total <= size_mm * f64::from(count) * 1.5 {
        return segments.to_vec();
    }
    let spacing = total / f64::from(count);
    let half = size_mm / 2.0;
    let gaps: Vec<(f64, f64)> = (0..count)
        .map(|i| {
            let c = (f64::from(i) + 0.5) * spacing;
            (c - half, c + half)
        })
        .collect();
    // Subtract every gap interval from [a, b], in arc length.
    let keep = |a: f64, b: f64| -> Vec<(f64, f64)> {
        let mut spans = vec![(a, b)];
        for &(gl, gh) in &gaps {
            let mut next = Vec::new();
            for (x, y) in spans {
                if gh <= x || gl >= y {
                    next.push((x, y));
                } else {
                    if x < gl {
                        next.push((x, gl.min(y)));
                    }
                    if gh < y {
                        next.push((gh.max(x), y));
                    }
                }
            }
            spans = next;
        }
        spans
    };
    let mut out = Vec::new();
    let mut acc = 0.0;
    for s in segments {
        let (dx, dy) = (s.end.x - s.start.x, s.end.y - s.start.y);
        let len = dx.hypot(dy);
        if len <= f64::EPSILON {
            continue;
        }
        let (ux, uy) = (dx / len, dy / len);
        for (a, b) in keep(acc, acc + len) {
            let (la, lb) = (a - acc, b - acc);
            out.push(Segment {
                start: Point {
                    x: s.start.x + ux * la,
                    y: s.start.y + uy * la,
                },
                end: Point {
                    x: s.start.x + ux * lb,
                    y: s.start.y + uy * lb,
                },
            });
        }
        acc += len;
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
    fn hatch_fill_angled_turns_lines_vertical_at_ninety() {
        let square = [
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
            Point { x: 0.0, y: 10.0 },
        ];
        let lines = hatch_fill_angled(&square, 1.0, 90.0, false);
        assert_eq!(lines.len(), 10);
        // Every line now runs vertically: equal x on both ends.
        for seg in &lines {
            assert!((seg.start.x - seg.end.x).abs() < 1e-6, "not vertical");
        }
    }

    #[test]
    fn cross_hatch_doubles_the_line_count() {
        let square = [
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
            Point { x: 0.0, y: 10.0 },
        ];
        let single = hatch_fill_angled(&square, 1.0, 0.0, false);
        let crossed = hatch_fill_angled(&square, 1.0, 0.0, true);
        assert_eq!(crossed.len(), single.len() * 2);
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

    #[test]
    fn perforate_dashes_a_straight_run() {
        // A 10 mm line with 1 mm cut and 1 mm skip gives dashes at
        // 0..1, 2..3, 4..5, 6..7, 8..9: five dashes.
        let line = [Point { x: 0.0, y: 0.0 }, Point { x: 10.0, y: 0.0 }];
        let dashes = perforate_path(&line, false, 1.0, 1.0);
        assert_eq!(dashes.len(), 5);
        assert!((dashes[0].start.x - 0.0).abs() < 1e-9);
        assert!((dashes[0].end.x - 1.0).abs() < 1e-9);
        assert!((dashes[1].start.x - 2.0).abs() < 1e-9);
        // Total cut length is five dashes of 1 mm each.
        let cut: f64 = dashes.iter().map(|s| (s.end.x - s.start.x).abs()).sum();
        assert!((cut - 5.0).abs() < 1e-9, "cut {cut}");
    }

    #[test]
    fn perforate_passes_solid_through_on_bad_params() {
        let line = [Point { x: 0.0, y: 0.0 }, Point { x: 10.0, y: 0.0 }];
        assert_eq!(perforate_path(&line, false, 0.0, 1.0).len(), 1);
        assert_eq!(perforate_path(&line, false, 1.0, 0.0).len(), 1);
    }

    #[test]
    fn overcut_extends_past_the_start() {
        let square = [
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
            Point { x: 0.0, y: 10.0 },
        ];
        // After closing to (0,0) the head continues onto the first edge toward
        // (10,0); a 3 mm overcut lands at (3,0).
        let extra = overcut_segments(&square, true, 3.0);
        let total: f64 = extra
            .iter()
            .map(|s| (s.end.x - s.start.x).hypot(s.end.y - s.start.y))
            .sum();
        assert!((total - 3.0).abs() < 1e-9, "total {total}");
        let last = extra.last().expect("a segment");
        assert!((last.end.x - 3.0).abs() < 1e-9 && last.end.y.abs() < 1e-9);
    }

    #[test]
    fn overcut_skips_open_paths() {
        let line = [Point { x: 0.0, y: 0.0 }, Point { x: 10.0, y: 0.0 }];
        assert!(overcut_segments(&line, false, 3.0).is_empty());
        assert!(overcut_segments(&line, true, 0.0).is_empty());
    }

    #[test]
    fn lead_in_approaches_the_start_from_outside() {
        let square = [
            Point { x: 2.0, y: 0.0 },
            Point { x: 12.0, y: 0.0 },
            Point { x: 12.0, y: 10.0 },
        ];
        // First edge heads +x, so the lead-in starts 2 mm back at (0,0) and ends
        // at the start vertex (2,0).
        let li = lead_in_segment(&square, 2.0).expect("a lead-in");
        assert!((li.start.x - 0.0).abs() < 1e-9 && li.start.y.abs() < 1e-9);
        assert!((li.end.x - 2.0).abs() < 1e-9);
    }

    #[test]
    fn lead_out_continues_past_the_end() {
        let line = [Point { x: 0.0, y: 0.0 }, Point { x: 10.0, y: 0.0 }];
        // Open path: lead-out continues +x past (10,0) by 2 mm to (12,0).
        let lo = lead_out_segment(&line, false, 2.0).expect("a lead-out");
        assert!((lo.start.x - 10.0).abs() < 1e-9);
        assert!((lo.end.x - 12.0).abs() < 1e-9 && lo.end.y.abs() < 1e-9);
        assert!(lead_in_segment(&line, 0.0).is_none());
    }

    #[test]
    fn tabs_remove_their_length_from_the_outline() {
        let square = [
            Point { x: 0.0, y: 0.0 },
            Point { x: 40.0, y: 0.0 },
            Point { x: 40.0, y: 40.0 },
            Point { x: 0.0, y: 40.0 },
        ];
        let outline = follow_polyline(&square, true);
        let solid: f64 = outline
            .iter()
            .map(|s| (s.end.x - s.start.x).hypot(s.end.y - s.start.y))
            .sum();
        let tabbed = apply_tabs(&outline, 4, 2.0);
        let cut: f64 = tabbed
            .iter()
            .map(|s| (s.end.x - s.start.x).hypot(s.end.y - s.start.y))
            .sum();
        // Four tabs of 2 mm remove 8 mm from the 160 mm perimeter.
        assert!((solid - cut - 8.0).abs() < 1e-6, "cut {cut} solid {solid}");
        // The cut is now broken into more pieces than the four original edges.
        assert!(tabbed.len() > outline.len(), "pieces {}", tabbed.len());
    }

    #[test]
    fn tabs_pass_through_when_disabled_or_too_big() {
        let square = [
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
            Point { x: 0.0, y: 10.0 },
        ];
        let outline = follow_polyline(&square, true);
        assert_eq!(apply_tabs(&outline, 0, 2.0).len(), outline.len());
        assert_eq!(apply_tabs(&outline, 4, 0.0).len(), outline.len());
        // A 20 mm tab on a 40 mm perimeter would consume it: left unchanged.
        assert_eq!(apply_tabs(&outline, 4, 20.0).len(), outline.len());
    }
}
