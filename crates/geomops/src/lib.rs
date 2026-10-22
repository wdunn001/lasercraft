#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! Polygon boolean and offset for LaserCraft.
//!
//! LaserCraft's document is anchor-free polylines, so these functions convert to
//! `craft-geom`'s [`PathData`], run the shared `craft-pathops` algorithms, and
//! flatten the result back to polylines. Weld unions overlapping outlines into
//! one, which a cut then traces as a single clean edge. Concentric offsets an
//! outline inward in steps, the true offset fill for an arbitrary shape.

use craft_geom::kurbo::{PathEl, flatten as kurbo_flatten};
use craft_geom::{FillRule, PathData, Point, SubPath};
use craft_pathops::{BoolOp, Join, boolean, offset_path};

/// Build a closed path from a polyline given as `(x, y)` pairs in millimetres.
fn to_path(points: &[(f64, f64)]) -> PathData {
    let pts: Vec<Point> = points.iter().map(|&(x, y)| Point::new(x, y)).collect();
    PathData::single(SubPath::polyline(&pts, true))
}

/// Flatten a path to one polyline per subpath, at a 0.05 mm chord tolerance.
fn to_polylines(path: &PathData) -> Vec<Vec<(f64, f64)>> {
    let mut subs: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut current: Vec<(f64, f64)> = Vec::new();
    kurbo_flatten(path.to_bezpath(), 0.05, |el| match el {
        PathEl::MoveTo(p) => {
            if !current.is_empty() {
                subs.push(std::mem::take(&mut current));
            }
            current.push((p.x, p.y));
        }
        PathEl::LineTo(p) => current.push((p.x, p.y)),
        PathEl::ClosePath if !current.is_empty() => subs.push(std::mem::take(&mut current)),
        _ => {}
    });
    if !current.is_empty() {
        subs.push(current);
    }
    subs
}

/// Union a set of closed polylines into their merged outline.
///
/// Overlapping shapes become one outline. The result is one polyline per
/// resulting subpath, so an outline with a hole comes back as two.
pub fn weld(polylines: &[Vec<(f64, f64)>]) -> Vec<Vec<(f64, f64)>> {
    let Some((first, rest)) = polylines.split_first() else {
        return Vec::new();
    };
    let mut acc = to_path(first);
    for pl in rest {
        acc = boolean(
            &acc,
            FillRule::NonZero,
            &to_path(pl),
            FillRule::NonZero,
            BoolOp::Union,
        );
    }
    to_polylines(&acc)
}

/// Subtract `cutter` from `base`, both closed polylines (`base` minus `cutter`).
pub fn difference(base: &[(f64, f64)], cutter: &[(f64, f64)]) -> Vec<Vec<(f64, f64)>> {
    let out = boolean(
        &to_path(base),
        FillRule::NonZero,
        &to_path(cutter),
        FillRule::NonZero,
        BoolOp::Difference,
    );
    to_polylines(&out)
}

/// Intersect two closed polylines, keeping only the overlapping region.
pub fn intersect(a: &[(f64, f64)], b: &[(f64, f64)]) -> Vec<Vec<(f64, f64)>> {
    let out = boolean(
        &to_path(a),
        FillRule::NonZero,
        &to_path(b),
        FillRule::NonZero,
        BoolOp::Intersect,
    );
    to_polylines(&out)
}

/// Offset a closed outline by one signed distance. Positive grows the outline
/// outward, negative shrinks it inward. This is the Offset Shapes operation, a
/// cut line set a fixed distance off an outline. The result is one polyline per
/// resulting subpath, and a collapsed offset comes back empty.
pub fn offset(outline: &[(f64, f64)], delta: f64) -> Vec<Vec<(f64, f64)>> {
    if outline.len() < 3 || delta == 0.0 {
        return Vec::new();
    }
    to_polylines(&offset_path(&to_path(outline), delta, Join::Round, 2.0))
        .into_iter()
        .filter(|s| s.len() >= 3)
        .collect()
}

/// Offset a closed outline inward in `interval` steps, up to `max_rings` rings.
///
/// This is the offset fill for an arbitrary shape. Each ring is the outline inset
/// by one more interval. Rings stop once an offset collapses to nothing.
pub fn concentric(outline: &[(f64, f64)], interval: f64, max_rings: usize) -> Vec<Vec<(f64, f64)>> {
    let mut rings: Vec<Vec<(f64, f64)>> = Vec::new();
    if interval <= 0.0 || outline.len() < 3 {
        return rings;
    }
    let base = to_path(outline);
    for i in 0..max_rings {
        let delta = -(interval * (i as f64 + 1.0));
        let ring = to_polylines(&offset_path(&base, delta, Join::Round, 2.0));
        let kept: Vec<Vec<(f64, f64)>> = ring.into_iter().filter(|s| s.len() >= 3).collect();
        if kept.is_empty() {
            break;
        }
        rings.extend(kept);
    }
    rings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weld_merges_two_overlapping_squares() {
        let a = vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)];
        let b = vec![(5.0, 5.0), (15.0, 5.0), (15.0, 15.0), (5.0, 15.0)];
        let out = weld(&[a, b]);
        // The union is one outline with more than the four points of a square.
        assert_eq!(out.len(), 1, "subpaths {}", out.len());
        assert!(out[0].len() >= 6, "points {}", out[0].len());
    }

    #[test]
    fn difference_and_intersect_two_squares() {
        let a = vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)];
        let b = vec![(5.0, 5.0), (15.0, 5.0), (15.0, 15.0), (5.0, 15.0)];
        assert!(!difference(&a, &b).is_empty());
        assert!(!intersect(&a, &b).is_empty());
    }

    #[test]
    fn concentric_rings_a_square() {
        let sq = vec![(0.0, 0.0), (20.0, 0.0), (20.0, 20.0), (0.0, 20.0)];
        let rings = concentric(&sq, 2.0, 10);
        // A 20 mm square inset by 2 mm gives several rings before it collapses.
        assert!(rings.len() >= 3, "rings {}", rings.len());
    }

    #[test]
    fn offset_outset_grows_and_inset_shrinks() {
        let sq = vec![(0.0, 0.0), (20.0, 0.0), (20.0, 20.0), (0.0, 20.0)];
        let out = offset(&sq, 3.0);
        let inn = offset(&sq, -3.0);
        assert!(!out.is_empty() && !inn.is_empty());
        // The outset ring reaches beyond the 20 mm square; the inset stays inside.
        let out_max = out.iter().flatten().fold(f64::MIN, |m, &(x, _)| m.max(x));
        let inn_max = inn.iter().flatten().fold(f64::MIN, |m, &(x, _)| m.max(x));
        assert!(out_max > 20.5, "outset max x {out_max}");
        assert!(inn_max < 19.5, "inset max x {inn_max}");
    }

    #[test]
    fn degenerate_input_is_safe() {
        assert!(weld(&[]).is_empty());
        assert!(offset(&[(0.0, 0.0), (1.0, 1.0)], 2.0).is_empty());
        assert!(offset(&[(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)], 0.0).is_empty());
        assert!(concentric(&[(0.0, 0.0)], 1.0, 5).is_empty());
        assert!(concentric(&[(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)], 0.0, 5).is_empty());
    }
}
