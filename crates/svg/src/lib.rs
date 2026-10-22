#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! SVG path import for LaserCraft.
//!
//! SVG is the universal vector interchange format, so importing its path data is
//! how real art enters LaserCraft. This crate parses the `d` attribute of an SVG
//! path into [`BezierPath`] subpaths. It handles the move, line, horizontal,
//! vertical, cubic, quadratic, and close commands in both absolute and relative
//! forms. Quadratics are elevated to cubics. Smooth shorthands (`S`, `T`), arcs
//! (`A`), transforms, and units are later work. Unknown commands return an error
//! rather than silently dropping geometry.

use lasercraft_laser_doc::{BezierPath, PathSeg, Point2};

/// Parse SVG path data into one or more subpaths.
///
/// Each `M` starts a new subpath. A `Z` closes the current subpath. Coordinates
/// outside any subpath, or an unsupported command, produce an error.
pub fn parse_path_data(data: &str) -> Result<Vec<BezierPath>, &'static str> {
    let bytes = data.as_bytes();
    let mut pos = 0usize;
    let mut paths: Vec<BezierPath> = Vec::new();
    let mut current: Option<BezierPath> = None;
    let mut point = Point2 { x: 0.0, y: 0.0 };
    let mut start = Point2 { x: 0.0, y: 0.0 };
    let mut last_command = 0u8;

    loop {
        skip_separators(bytes, &mut pos);
        let Some(&byte) = bytes.get(pos) else { break };
        let command = if byte.is_ascii_alphabetic() {
            pos += 1;
            byte
        } else if last_command != 0 {
            // An implicit repeat of the previous command. A repeated move becomes
            // a line, matching the SVG rule.
            match last_command {
                b'M' => b'L',
                b'm' => b'l',
                other => other,
            }
        } else {
            return Err("path data must begin with a move command");
        };
        last_command = command;
        let absolute = command.is_ascii_uppercase();

        match command.to_ascii_uppercase() {
            b'M' => {
                let (x, y) = read_pair(bytes, &mut pos)?;
                point = resolve(absolute, point, x, y);
                start = point;
                if let Some(path) = current.take() {
                    paths.push(path);
                }
                current = Some(BezierPath {
                    start: point,
                    segments: Vec::new(),
                    closed: false,
                });
            }
            b'L' => {
                let (x, y) = read_pair(bytes, &mut pos)?;
                point = resolve(absolute, point, x, y);
                push_seg(&mut current, PathSeg::Line { to: point })?;
            }
            b'H' => {
                let x = read_number(bytes, &mut pos)?;
                point = Point2 {
                    x: if absolute { x } else { point.x + x },
                    y: point.y,
                };
                push_seg(&mut current, PathSeg::Line { to: point })?;
            }
            b'V' => {
                let y = read_number(bytes, &mut pos)?;
                point = Point2 {
                    x: point.x,
                    y: if absolute { y } else { point.y + y },
                };
                push_seg(&mut current, PathSeg::Line { to: point })?;
            }
            b'C' => {
                let (c1x, c1y) = read_pair(bytes, &mut pos)?;
                let (c2x, c2y) = read_pair(bytes, &mut pos)?;
                let (x, y) = read_pair(bytes, &mut pos)?;
                let c1 = resolve(absolute, point, c1x, c1y);
                let c2 = resolve(absolute, point, c2x, c2y);
                point = resolve(absolute, point, x, y);
                push_seg(&mut current, PathSeg::Cubic { c1, c2, to: point })?;
            }
            b'Q' => {
                let (qx, qy) = read_pair(bytes, &mut pos)?;
                let (x, y) = read_pair(bytes, &mut pos)?;
                let q = resolve(absolute, point, qx, qy);
                let end = resolve(absolute, point, x, y);
                // Elevate the quadratic to an equivalent cubic.
                let c1 = Point2 {
                    x: point.x + 2.0 / 3.0 * (q.x - point.x),
                    y: point.y + 2.0 / 3.0 * (q.y - point.y),
                };
                let c2 = Point2 {
                    x: end.x + 2.0 / 3.0 * (q.x - end.x),
                    y: end.y + 2.0 / 3.0 * (q.y - end.y),
                };
                point = end;
                push_seg(&mut current, PathSeg::Cubic { c1, c2, to: point })?;
            }
            b'A' => {
                let rx = read_number(bytes, &mut pos)?;
                let ry = read_number(bytes, &mut pos)?;
                let rotation = read_number(bytes, &mut pos)?;
                let large_arc = read_number(bytes, &mut pos)? != 0.0;
                let sweep = read_number(bytes, &mut pos)? != 0.0;
                let (x, y) = read_pair(bytes, &mut pos)?;
                let end = resolve(absolute, point, x, y);
                for (c1, c2, to) in arc_to_cubics(point, rx, ry, rotation, large_arc, sweep, end) {
                    push_seg(&mut current, PathSeg::Cubic { c1, c2, to })?;
                }
                point = end;
            }
            b'Z' => {
                if let Some(path) = current.as_mut() {
                    path.closed = true;
                }
                if let Some(path) = current.take() {
                    paths.push(path);
                }
                point = start;
            }
            _ => return Err("unsupported SVG path command"),
        }
    }
    if let Some(path) = current.take() {
        paths.push(path);
    }
    Ok(paths)
}

/// Convert an SVG elliptical arc to a sequence of cubic bezier segments.
///
/// Follows the SVG implementation notes: convert the endpoint parameterization to
/// the centre parameterization, then approximate the sweep with cubics of at most
/// ninety degrees each. A zero radius or a zero-length arc degenerates to a single
/// straight cubic to the endpoint.
fn arc_to_cubics(
    from: Point2,
    rx: f64,
    ry: f64,
    rotation_deg: f64,
    large_arc: bool,
    sweep: bool,
    to: Point2,
) -> Vec<(Point2, Point2, Point2)> {
    use std::f64::consts::PI;
    let line = vec![(from, to, to)];
    if rx == 0.0 || ry == 0.0 || (from.x == to.x && from.y == to.y) {
        return line;
    }
    let mut rx = rx.abs();
    let mut ry = ry.abs();
    let phi = rotation_deg.to_radians();
    let (sin_phi, cos_phi) = (phi.sin(), phi.cos());

    let dx = (from.x - to.x) / 2.0;
    let dy = (from.y - to.y) / 2.0;
    let x1p = cos_phi * dx + sin_phi * dy;
    let y1p = -sin_phi * dx + cos_phi * dy;

    let lambda = x1p * x1p / (rx * rx) + y1p * y1p / (ry * ry);
    if lambda > 1.0 {
        let scale = lambda.sqrt();
        rx *= scale;
        ry *= scale;
    }

    let num = (rx * rx * ry * ry - rx * rx * y1p * y1p - ry * ry * x1p * x1p).max(0.0);
    let den = rx * rx * y1p * y1p + ry * ry * x1p * x1p;
    let mut coef = if den == 0.0 { 0.0 } else { (num / den).sqrt() };
    if large_arc == sweep {
        coef = -coef;
    }
    let cxp = coef * rx * y1p / ry;
    let cyp = -coef * ry * x1p / rx;
    let cx = cos_phi * cxp - sin_phi * cyp + (from.x + to.x) / 2.0;
    let cy = sin_phi * cxp + cos_phi * cyp + (from.y + to.y) / 2.0;

    let angle = |ux: f64, uy: f64, vx: f64, vy: f64| -> f64 {
        let dot = ux * vx + uy * vy;
        let len = (ux * ux + uy * uy).sqrt() * (vx * vx + vy * vy).sqrt();
        let base = if len == 0.0 {
            0.0
        } else {
            (dot / len).clamp(-1.0, 1.0).acos()
        };
        if ux * vy - uy * vx < 0.0 { -base } else { base }
    };

    let ux = (x1p - cxp) / rx;
    let uy = (y1p - cyp) / ry;
    let theta1 = angle(1.0, 0.0, ux, uy);
    let mut dtheta = angle(ux, uy, (-x1p - cxp) / rx, (-y1p - cyp) / ry);
    if !sweep && dtheta > 0.0 {
        dtheta -= 2.0 * PI;
    } else if sweep && dtheta < 0.0 {
        dtheta += 2.0 * PI;
    }

    let segments = (dtheta.abs() / (PI / 2.0)).ceil().max(1.0);
    let count = segments as usize;
    let delta = dtheta / segments;
    let k = 4.0 / 3.0 * (delta / 4.0).tan();

    let point_at = |a: f64| -> Point2 {
        let (sin_a, cos_a) = (a.sin(), a.cos());
        Point2 {
            x: cx + rx * cos_a * cos_phi - ry * sin_a * sin_phi,
            y: cy + rx * cos_a * sin_phi + ry * sin_a * cos_phi,
        }
    };
    let derivative_at = |a: f64| -> (f64, f64) {
        let (sin_a, cos_a) = (a.sin(), a.cos());
        (
            -rx * sin_a * cos_phi - ry * cos_a * sin_phi,
            -rx * sin_a * sin_phi + ry * cos_a * cos_phi,
        )
    };

    let mut result = Vec::with_capacity(count);
    let mut t = theta1;
    let mut start = point_at(t);
    for _ in 0..count {
        let t2 = t + delta;
        let end = point_at(t2);
        let (d1x, d1y) = derivative_at(t);
        let (d2x, d2y) = derivative_at(t2);
        let c1 = Point2 {
            x: start.x + k * d1x,
            y: start.y + k * d1y,
        };
        let c2 = Point2 {
            x: end.x - k * d2x,
            y: end.y - k * d2y,
        };
        result.push((c1, c2, end));
        t = t2;
        start = end;
    }
    result
}

fn resolve(absolute: bool, from: Point2, x: f64, y: f64) -> Point2 {
    if absolute {
        Point2 { x, y }
    } else {
        Point2 {
            x: from.x + x,
            y: from.y + y,
        }
    }
}

fn push_seg(current: &mut Option<BezierPath>, seg: PathSeg) -> Result<(), &'static str> {
    match current.as_mut() {
        Some(path) => {
            path.segments.push(seg);
            Ok(())
        }
        None => Err("path segment before a move command"),
    }
}

fn is_separator(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | b',')
}

fn skip_separators(bytes: &[u8], pos: &mut usize) {
    while let Some(&b) = bytes.get(*pos) {
        if is_separator(b) {
            *pos += 1;
        } else {
            break;
        }
    }
}

fn read_pair(bytes: &[u8], pos: &mut usize) -> Result<(f64, f64), &'static str> {
    let x = read_number(bytes, pos)?;
    let y = read_number(bytes, pos)?;
    Ok((x, y))
}

/// Read one SVG number, following the rule that a sign or a second decimal point
/// starts a new number even without a separator.
fn read_number(bytes: &[u8], pos: &mut usize) -> Result<f64, &'static str> {
    skip_separators(bytes, pos);
    let start = *pos;
    if matches!(bytes.get(*pos), Some(b'+' | b'-')) {
        *pos += 1;
    }
    let mut seen_dot = false;
    let mut seen_digit = false;
    while let Some(&b) = bytes.get(*pos) {
        match b {
            b'0'..=b'9' => {
                seen_digit = true;
                *pos += 1;
            }
            b'.' if !seen_dot => {
                seen_dot = true;
                *pos += 1;
            }
            b'e' | b'E' => {
                *pos += 1;
                if matches!(bytes.get(*pos), Some(b'+' | b'-')) {
                    *pos += 1;
                }
            }
            _ => break,
        }
    }
    if !seen_digit {
        return Err("expected a number in path data");
    }
    let text = std::str::from_utf8(&bytes[start..*pos]).map_err(|_| "non-ascii number")?;
    text.parse::<f64>()
        .map_err(|_| "invalid number in path data")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_an_absolute_closed_triangle() {
        let paths = parse_path_data("M0 0 L10 0 L5 8 Z").unwrap_or_default();
        assert_eq!(paths.len(), 1);
        if let Some(path) = paths.first() {
            assert_eq!(path.start, Point2 { x: 0.0, y: 0.0 });
            assert_eq!(path.segments.len(), 2);
            assert!(path.closed);
        }
    }

    #[test]
    fn relative_and_hv_commands_track_the_point() {
        // m moves to (1,1); h10 -> (11,1); v5 -> (11,6); l-1 -1 -> (10,5).
        let paths = parse_path_data("m1 1 h10 v5 l-1 -1").unwrap_or_default();
        assert_eq!(paths.len(), 1);
        if let Some(path) = paths.first() {
            assert_eq!(path.start, Point2 { x: 1.0, y: 1.0 });
            assert_eq!(path.segments.len(), 3);
            assert_eq!(
                path.segments.last().copied(),
                Some(PathSeg::Line {
                    to: Point2 { x: 10.0, y: 5.0 }
                })
            );
        }
    }

    #[test]
    fn two_subpaths_split_on_move() {
        let paths = parse_path_data("M0 0 L1 0 M2 2 L3 2").unwrap_or_default();
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn implicit_lineto_after_move() {
        // After "M0 0" the next pair is an implicit L to (1,0), then (2,0).
        let paths = parse_path_data("M0 0 1 0 2 0").unwrap_or_default();
        assert_eq!(paths.len(), 1);
        if let Some(path) = paths.first() {
            assert_eq!(path.segments.len(), 2);
        }
    }

    #[test]
    fn rejects_segment_before_move() {
        assert!(parse_path_data("L10 10").is_err());
    }

    #[test]
    fn arc_approximates_a_unit_quarter_circle() {
        // A quarter arc from (1,0) to (0,1), radius 1, centred at the origin.
        let paths = parse_path_data("M1 0 A1 1 0 0 1 0 1").unwrap_or_default();
        assert_eq!(paths.len(), 1);
        if let Some(path) = paths.first() {
            assert!(!path.segments.is_empty());
            // Every flattened point should sit on the unit circle within the
            // bezier approximation error for a ninety-degree arc.
            for p in path.flatten(16) {
                let r = (p.x * p.x + p.y * p.y).sqrt();
                assert!((r - 1.0).abs() < 0.01, "radius {r} off the unit circle");
            }
        }
    }
}
