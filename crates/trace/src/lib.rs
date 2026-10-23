#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! Bitmap to vector tracing for LaserCraft.
//!
//! LightBurn ships `potrace.dll`, which is Peter Selinger's open-source Potrace.
//! See `docs/rea-findings.md`. There is no reason to reverse engineer it. This
//! crate defines the LaserCraft-side contracts and a clean Potrace port implements
//! [`Tracer`] behind them. Defining the contracts first keeps the raster input and
//! the vector output independent of the algorithm.

/// A one-bit raster. A `true` pixel is part of the mark.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bitmap {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// Row-major pixels, `width * height` of them.
    pub pixels: Vec<bool>,
}

impl Bitmap {
    /// Read a pixel, returning `false` for out-of-bounds coordinates.
    pub fn get(&self, x: u32, y: u32) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        let index = (y as usize) * (self.width as usize) + (x as usize);
        self.pixels.get(index).copied().unwrap_or(false)
    }
}

/// Parameters that shape a trace.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TraceParams {
    /// Pixels at or above this value are treated as marks when thresholding.
    pub threshold: u8,
    /// Suppress speckles smaller than this many pixels, the Potrace turd size.
    pub speckle_suppression: u32,
    /// Corner smoothing from 0.0 (sharp) to 1.33 (round), the Potrace alphamax.
    pub corner_smoothing: f64,
}

impl Default for TraceParams {
    fn default() -> Self {
        Self {
            threshold: 128,
            speckle_suppression: 2,
            corner_smoothing: 1.0,
        }
    }
}

/// A traced point in bitmap pixel coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
}

/// A traced contour.
#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    /// The contour points in order.
    pub points: Vec<Point>,
    /// Whether the contour closes back to its first point.
    pub closed: bool,
}

/// A bitmap tracer. The Potrace port implements this.
pub trait Tracer {
    /// Trace a bitmap into vector contours.
    fn trace(&self, bitmap: &Bitmap, params: &TraceParams) -> Vec<Path>;
}

/// Clockwise Moore-neighbourhood offsets, starting east, with y growing down.
const MOORE: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

/// A boundary-following tracer. It walks the edge of each mark region into a
/// closed contour, suppresses speckles by area and simplifies the staircase with
/// Douglas-Peucker. It is not Potrace, which fits beziers, but it turns a bitmap
/// into clean closed polylines a laser can cut, which is the job here.
#[derive(Debug, Default, Clone, Copy)]
pub struct BoundaryTracer;

impl Tracer for BoundaryTracer {
    fn trace(&self, bitmap: &Bitmap, params: &TraceParams) -> Vec<Path> {
        let w = i64::from(bitmap.width);
        let h = i64::from(bitmap.height);
        let get = |x: i64, y: i64| -> bool {
            x >= 0 && y >= 0 && x < w && y < h && bitmap.get(x as u32, y as u32)
        };
        let mut visited: std::collections::HashSet<(i64, i64)> = std::collections::HashSet::new();
        let epsilon = params.corner_smoothing.max(0.5) + 0.25;
        let min_area = f64::from(params.speckle_suppression.max(1));
        let mut out: Vec<Path> = Vec::new();
        for sy in 0..h {
            for sx in 0..w {
                if !get(sx, sy) || get(sx - 1, sy) || visited.contains(&(sx, sy)) {
                    continue;
                }
                let contour = trace_contour(sx, sy, &get);
                for &(x, y) in &contour {
                    visited.insert((x as i64, y as i64));
                }
                if contour.len() < 4 || polygon_area(&contour) < min_area {
                    continue;
                }
                out.push(Path {
                    points: simplify(&contour, epsilon)
                        .into_iter()
                        .map(|(x, y)| Point { x, y })
                        .collect(),
                    closed: true,
                });
            }
        }
        out
    }
}

/// Follow the boundary of a mark region clockwise from `(sx, sy)` back to the
/// start. A stuck trace stops after a generous step budget.
fn trace_contour(sx: i64, sy: i64, get: &impl Fn(i64, i64) -> bool) -> Vec<(f64, f64)> {
    let mut contour: Vec<(f64, f64)> = vec![(sx as f64, sy as f64)];
    let mut b = (sx, sy);
    let mut backtrack = (sx - 1, sy);
    let mut steps: u64 = 0;
    loop {
        steps += 1;
        if steps > 8_000_000 {
            break;
        }
        let bd = (backtrack.0 - b.0, backtrack.1 - b.1);
        let start = MOORE.iter().position(|&o| o == bd).unwrap_or(4);
        let mut prev = backtrack;
        let mut found: Option<((i64, i64), (i64, i64))> = None;
        for k in 1..=8 {
            let idx = (start + k) % 8;
            let n = (b.0 + MOORE[idx].0, b.1 + MOORE[idx].1);
            if get(n.0, n.1) {
                found = Some((n, prev));
                break;
            }
            prev = n;
        }
        match found {
            None => break,
            Some((n, bk)) => {
                b = n;
                backtrack = bk;
                if b == (sx, sy) {
                    break;
                }
                contour.push((b.0 as f64, b.1 as f64));
            }
        }
    }
    contour
}

/// The signed shoelace area magnitude of a polygon.
fn polygon_area(poly: &[(f64, f64)]) -> f64 {
    let n = poly.len();
    if n < 3 {
        return 0.0;
    }
    let mut sum = 0.0;
    let mut j = n - 1;
    for i in 0..n {
        if let (Some(&a), Some(&b)) = (poly.get(j), poly.get(i)) {
            sum += (a.0 + b.0) * (a.1 - b.1);
        }
        j = i;
    }
    (sum / 2.0).abs()
}

/// Simplify a polyline with the iterative Douglas-Peucker algorithm.
fn simplify(points: &[(f64, f64)], epsilon: f64) -> Vec<(f64, f64)> {
    let n = points.len();
    if n < 3 {
        return points.to_vec();
    }
    let mut keep = vec![false; n];
    // Keep the endpoints explicitly.
    if let Some(f) = keep.first_mut() {
        *f = true;
    }
    if let Some(l) = keep.last_mut() {
        *l = true;
    }
    let mut stack = vec![(0usize, n - 1)];
    while let Some((lo, hi)) = stack.pop() {
        if hi <= lo + 1 {
            continue;
        }
        let (Some(&a), Some(&b)) = (points.get(lo), points.get(hi)) else {
            continue;
        };
        let mut best = 0.0;
        let mut best_i = lo;
        for i in (lo + 1)..hi {
            if let Some(&p) = points.get(i) {
                let d = perp_distance(p, a, b);
                if d > best {
                    best = d;
                    best_i = i;
                }
            }
        }
        if best > epsilon {
            if let Some(k) = keep.get_mut(best_i) {
                *k = true;
            }
            stack.push((lo, best_i));
            stack.push((best_i, hi));
        }
    }
    points
        .iter()
        .zip(keep.iter())
        .filter_map(|(p, &k)| if k { Some(*p) } else { None })
        .collect()
}

/// Perpendicular distance from `p` to the line through `a` and `b`.
fn perp_distance(p: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let len = (dx * dx + dy * dy).sqrt();
    if len < 1e-9 {
        return ((p.0 - a.0).powi(2) + (p.1 - a.1).powi(2)).sqrt();
    }
    ((p.0 - a.0) * dy - (p.1 - a.1) * dx).abs() / len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitmap_get_is_bounds_safe() {
        let bitmap = Bitmap {
            width: 2,
            height: 2,
            pixels: vec![true, false, false, true],
        };
        assert!(bitmap.get(0, 0));
        assert!(bitmap.get(1, 1));
        assert!(!bitmap.get(1, 0));
        assert!(!bitmap.get(5, 5));
    }

    #[test]
    fn default_params_match_potrace_conventions() {
        let params = TraceParams::default();
        assert_eq!(params.threshold, 128);
        assert_eq!(params.speckle_suppression, 2);
    }

    fn filled_square(w: u32, h: u32, x0: u32, y0: u32, x1: u32, y1: u32) -> Bitmap {
        let mut pixels = vec![false; (w * h) as usize];
        for y in y0..y1 {
            for x in x0..x1 {
                if let Some(slot) = pixels.get_mut((y * w + x) as usize) {
                    *slot = true;
                }
            }
        }
        Bitmap {
            width: w,
            height: h,
            pixels,
        }
    }

    #[test]
    fn traces_a_square_into_one_closed_contour() {
        let bitmap = filled_square(10, 10, 2, 2, 8, 8);
        let paths = BoundaryTracer.trace(&bitmap, &TraceParams::default());
        assert_eq!(paths.len(), 1, "paths {}", paths.len());
        if let Some(path) = paths.first() {
            assert!(path.closed);
            // A 6x6 filled square simplifies to roughly its four corners.
            assert!(
                path.points.len() >= 4 && path.points.len() <= 8,
                "points {}",
                path.points.len()
            );
            let min_x = path.points.iter().fold(f64::MAX, |m, p| m.min(p.x));
            let max_x = path.points.iter().fold(f64::MIN, |m, p| m.max(p.x));
            assert!(min_x <= 2.5 && max_x >= 6.5, "x span {min_x}..{max_x}");
        }
    }

    #[test]
    fn speckle_suppression_drops_a_tiny_dot() {
        // A single lone pixel is below the default speckle area and is dropped.
        let bitmap = filled_square(10, 10, 5, 5, 6, 6);
        let paths = BoundaryTracer.trace(&bitmap, &TraceParams::default());
        assert!(
            paths.is_empty(),
            "a 1px dot should be suppressed, got {}",
            paths.len()
        );
    }

    #[test]
    fn two_separate_squares_trace_as_two_contours() {
        let mut bitmap = filled_square(20, 10, 2, 2, 7, 8);
        let right = filled_square(20, 10, 12, 2, 17, 8);
        for (slot, on) in bitmap.pixels.iter_mut().zip(right.pixels.iter()) {
            *slot = *slot || *on;
        }
        let paths = BoundaryTracer.trace(&bitmap, &TraceParams::default());
        assert_eq!(paths.len(), 2, "paths {}", paths.len());
    }
}
