#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable,
    clippy::indexing_slicing
)]
//! A reader for ASCII DXF drawings.
//!
//! LightBurn imports CAD drawings, so LaserCraft reads the common ASCII DXF
//! entities and returns them as millimetre polylines the engine cuts. DXF is a
//! published format, so this is a clean-room reader built from the spec, not from
//! any LightBurn code.
//!
//! Supported entities are LINE, LWPOLYLINE (with arc bulges), the older POLYLINE
//! with its VERTEX list, CIRCLE and ARC. A circle or arc flattens to a polyline at
//! a fixed step. Binary DXF, splines, blocks and text are out of scope for now.
//! Coordinates pass through as drawing units, which laser work treats as
//! millimetres.

use std::f64::consts::PI;

/// A polyline read from the drawing, in drawing units.
#[derive(Debug, Clone, PartialEq)]
pub struct DxfPolyline {
    /// The vertices in order.
    pub points: Vec<(f64, f64)>,
    /// Whether the last point joins back to the first.
    pub closed: bool,
}

/// One group-code and value pair, the atom of an ASCII DXF file.
type Pair = (i32, String);

/// Split the text into group-code and value pairs. A line that is not an integer
/// code is skipped along with its value, so stray content cannot derail the read.
fn pairs(text: &str) -> Vec<Pair> {
    let mut lines = text.lines();
    let mut out = Vec::new();
    while let (Some(code_line), Some(value_line)) = (lines.next(), lines.next()) {
        if let Ok(code) = code_line.trim().parse::<i32>() {
            out.push((code, value_line.trim().to_string()));
        }
    }
    out
}

/// One entity: its type name and the pairs that belong to it.
struct Entity {
    kind: String,
    fields: Vec<Pair>,
}

/// Group the pairs into entities. A code of 0 starts a new entity, so every type
/// marker (including section and table markers) opens one. The caller acts only on
/// the entity kinds it knows.
fn entities(pairs: &[Pair]) -> Vec<Entity> {
    let mut out: Vec<Entity> = Vec::new();
    let mut current: Option<Entity> = None;
    for (code, value) in pairs {
        if *code == 0 {
            if let Some(entity) = current.take() {
                out.push(entity);
            }
            current = Some(Entity {
                kind: value.clone(),
                fields: Vec::new(),
            });
        } else if let Some(entity) = current.as_mut() {
            entity.fields.push((*code, value.clone()));
        }
    }
    if let Some(entity) = current.take() {
        out.push(entity);
    }
    out
}

/// The first float value for a group code in an entity.
fn float_of(entity: &Entity, code: i32) -> Option<f64> {
    entity
        .fields
        .iter()
        .find(|(c, _)| *c == code)
        .and_then(|(_, v)| v.parse::<f64>().ok())
}

/// The first integer value for a group code in an entity, defaulting to zero.
fn int_of(entity: &Entity, code: i32) -> i32 {
    entity
        .fields
        .iter()
        .find(|(c, _)| *c == code)
        .and_then(|(_, v)| v.trim().parse::<i32>().ok())
        .unwrap_or(0)
}

/// Steps used to flatten a full circle. An arc uses a share of this by angle.
const CIRCLE_STEPS: u32 = 64;

/// Flatten an arc centred at `(cx, cy)` from `start` through `sweep` radians.
///
/// The number of chords scales with the swept angle so a short arc stays light and
/// a long one stays smooth. The start point is included, so the caller appends the
/// run directly.
fn arc_points(cx: f64, cy: f64, radius: f64, start: f64, sweep: f64) -> Vec<(f64, f64)> {
    let steps = ((sweep.abs() / (2.0 * PI) * f64::from(CIRCLE_STEPS)).ceil() as u32).max(2);
    let mut pts = Vec::with_capacity(steps as usize + 1);
    for i in 0..=steps {
        let a = start + sweep * (f64::from(i) / f64::from(steps));
        pts.push((cx + radius * a.cos(), cy + radius * a.sin()));
    }
    pts
}

/// Append the arc that a bulge defines between `p1` and `p2`.
///
/// A DXF bulge is the tangent of a quarter of the arc's included angle, negative
/// for a clockwise sweep. Zero is a straight segment. The run omits `p1`, which the
/// caller has already placed, and ends on `p2`.
fn bulge_arc(p1: (f64, f64), p2: (f64, f64), bulge: f64) -> Vec<(f64, f64)> {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let chord = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
    if bulge.abs() < 1e-9 || chord < 1e-9 {
        return vec![p2];
    }
    let included = 4.0 * bulge.atan();
    let radius = chord / (2.0 * (included / 2.0).sin());
    // The centre lies on the chord's perpendicular, offset by the apothem. Using
    // the signed radius and the half-angle cosine puts it on the correct side.
    let nx = -(y2 - y1) / chord;
    let ny = (x2 - x1) / chord;
    let off = radius * (included / 2.0).cos();
    let cx = (x1 + x2) / 2.0 + nx * off;
    let cy = (y1 + y2) / 2.0 + ny * off;
    let start = (y1 - cy).atan2(x1 - cx);
    let mut pts = arc_points(cx, cy, radius.abs(), start, included);
    // Drop the first point, which repeats p1.
    if !pts.is_empty() {
        pts.remove(0);
    }
    pts
}

/// Read an LWPOLYLINE, honouring arc bulges between vertices.
fn read_lwpolyline(entity: &Entity) -> Option<DxfPolyline> {
    let closed = int_of(entity, 70) & 1 != 0;
    // Walk the fields in order so each 42 bulge attaches to the vertex before it.
    let mut verts: Vec<(f64, f64, f64)> = Vec::new();
    let mut pending_x: Option<f64> = None;
    for (code, value) in &entity.fields {
        match *code {
            10 => pending_x = value.parse::<f64>().ok(),
            20 => {
                if let (Some(x), Ok(y)) = (pending_x.take(), value.parse::<f64>()) {
                    verts.push((x, y, 0.0));
                }
            }
            42 => {
                if let (Some(last), Ok(b)) = (verts.last_mut(), value.parse::<f64>()) {
                    last.2 = b;
                }
            }
            _ => {}
        }
    }
    if verts.len() < 2 {
        return None;
    }
    let mut points: Vec<(f64, f64)> = Vec::new();
    if let Some(&(x, y, _)) = verts.first() {
        points.push((x, y));
    }
    for pair in verts.windows(2) {
        if let [a, b] = pair {
            points.extend(bulge_arc((a.0, a.1), (b.0, b.1), a.2));
        }
    }
    // A closed polyline may carry a bulge on its final vertex, back to the first.
    // The closing run ends on the first vertex, which the closed flag already
    // implies, so drop that duplicate and keep only any arc points before it.
    if closed && let (Some(&last), Some(&first)) = (verts.last(), verts.first()) {
        let mut closing = bulge_arc((last.0, last.1), (first.0, first.1), last.2);
        closing.pop();
        points.extend(closing);
    }
    Some(DxfPolyline { points, closed })
}

/// Read the older POLYLINE, whose vertices are the VERTEX entities that follow it.
///
/// `rest` is the entity slice starting at the first entity after the POLYLINE. The
/// return value is the polyline and the count of entities consumed, so the caller
/// advances past the VERTEX run and its SEQEND.
fn read_polyline(header: &Entity, rest: &[Entity]) -> (Option<DxfPolyline>, usize) {
    let closed = int_of(header, 70) & 1 != 0;
    let mut verts: Vec<(f64, f64, f64)> = Vec::new();
    let mut consumed = 0;
    for entity in rest {
        match entity.kind.as_str() {
            "VERTEX" => {
                consumed += 1;
                if let (Some(x), Some(y)) = (float_of(entity, 10), float_of(entity, 20)) {
                    verts.push((x, y, float_of(entity, 42).unwrap_or(0.0)));
                }
            }
            "SEQEND" => {
                consumed += 1;
                break;
            }
            _ => break,
        }
    }
    if verts.len() < 2 {
        return (None, consumed);
    }
    let mut points: Vec<(f64, f64)> = Vec::new();
    if let Some(&(x, y, _)) = verts.first() {
        points.push((x, y));
    }
    for pair in verts.windows(2) {
        if let [a, b] = pair {
            points.extend(bulge_arc((a.0, a.1), (b.0, b.1), a.2));
        }
    }
    if closed && let (Some(&last), Some(&first)) = (verts.last(), verts.first()) {
        let mut closing = bulge_arc((last.0, last.1), (first.0, first.1), last.2);
        closing.pop();
        points.extend(closing);
    }
    (Some(DxfPolyline { points, closed }), consumed)
}

/// Read every supported entity in `text` into polylines.
pub fn parse_dxf(text: &str) -> Vec<DxfPolyline> {
    let ents = entities(&pairs(text));
    let mut out: Vec<DxfPolyline> = Vec::new();
    let mut i = 0;
    while let Some(entity) = ents.get(i) {
        match entity.kind.as_str() {
            "LINE" => {
                if let (Some(x1), Some(y1), Some(x2), Some(y2)) = (
                    float_of(entity, 10),
                    float_of(entity, 20),
                    float_of(entity, 11),
                    float_of(entity, 21),
                ) {
                    out.push(DxfPolyline {
                        points: vec![(x1, y1), (x2, y2)],
                        closed: false,
                    });
                }
            }
            "CIRCLE" => {
                if let (Some(cx), Some(cy), Some(r)) = (
                    float_of(entity, 10),
                    float_of(entity, 20),
                    float_of(entity, 40),
                ) {
                    let mut points = arc_points(cx, cy, r, 0.0, 2.0 * PI);
                    points.pop(); // the closing flag replaces the duplicated first point
                    out.push(DxfPolyline {
                        points,
                        closed: true,
                    });
                }
            }
            "ARC" => {
                if let (Some(cx), Some(cy), Some(r), Some(a0), Some(a1)) = (
                    float_of(entity, 10),
                    float_of(entity, 20),
                    float_of(entity, 40),
                    float_of(entity, 50),
                    float_of(entity, 51),
                ) {
                    let start = a0 * PI / 180.0;
                    let mut end = a1 * PI / 180.0;
                    // DXF arcs sweep counter-clockwise from the start angle.
                    if end < start {
                        end += 2.0 * PI;
                    }
                    out.push(DxfPolyline {
                        points: arc_points(cx, cy, r, start, end - start),
                        closed: false,
                    });
                }
            }
            "LWPOLYLINE" => {
                if let Some(poly) = read_lwpolyline(entity) {
                    out.push(poly);
                }
            }
            "POLYLINE" => {
                let (poly, consumed) = read_polyline(entity, ents.get(i + 1..).unwrap_or(&[]));
                if let Some(poly) = poly {
                    out.push(poly);
                }
                i += consumed;
            }
            _ => {}
        }
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dxf(body: &str) -> String {
        format!("0\nSECTION\n2\nENTITIES\n{body}0\nENDSEC\n0\nEOF\n")
    }

    #[test]
    fn reads_a_line() {
        let out = parse_dxf(&dxf("0\nLINE\n10\n1.0\n20\n2.0\n11\n4.0\n21\n6.0\n"));
        assert_eq!(out.len(), 1);
        assert_eq!(
            out.first().map(|p| p.points.clone()),
            Some(vec![(1.0, 2.0), (4.0, 6.0)])
        );
    }

    #[test]
    fn reads_a_closed_lwpolyline() {
        let body = "0\nLWPOLYLINE\n90\n3\n70\n1\n10\n0\n20\n0\n10\n10\n20\n0\n10\n10\n20\n10\n";
        let out = parse_dxf(&dxf(body));
        assert_eq!(out.len(), 1);
        for poly in &out {
            assert!(poly.closed);
            assert_eq!(poly.points.len(), 3);
        }
    }

    #[test]
    fn circle_flattens_to_a_closed_ring_near_the_radius() {
        let out = parse_dxf(&dxf("0\nCIRCLE\n10\n0\n20\n0\n40\n5.0\n"));
        assert_eq!(out.len(), 1);
        for poly in &out {
            assert!(poly.closed);
            // Every point sits on the radius, within the chord error of the flattening.
            for &(x, y) in &poly.points {
                assert!(((x * x + y * y).sqrt() - 5.0).abs() < 0.1, "point {x},{y}");
            }
        }
    }

    #[test]
    fn arc_spans_from_start_to_end_angle() {
        // A quarter arc of radius 10 from 0 to 90 degrees runs (10,0) to (0,10).
        let out = parse_dxf(&dxf("0\nARC\n10\n0\n20\n0\n40\n10.0\n50\n0\n51\n90\n"));
        assert_eq!(out.len(), 1);
        for poly in &out {
            let first = poly.points.first().copied().unwrap_or((0.0, 0.0));
            let last = poly.points.last().copied().unwrap_or((0.0, 0.0));
            assert!(
                (first.0 - 10.0).abs() < 0.01 && first.1.abs() < 0.01,
                "start {first:?}"
            );
            assert!(
                last.0.abs() < 0.01 && (last.1 - 10.0).abs() < 0.01,
                "end {last:?}"
            );
        }
    }

    #[test]
    fn bulge_makes_a_semicircle() {
        // A bulge of 1 between (0,0) and (10,0) is a semicircle of radius 5.
        let body = "0\nLWPOLYLINE\n90\n2\n70\n0\n10\n0\n20\n0\n42\n1.0\n10\n10\n20\n0\n";
        let out = parse_dxf(&dxf(body));
        assert_eq!(out.len(), 1);
        for poly in &out {
            // The far point of the arc is radius 5 off the chord midpoint (5,0).
            let apex = poly
                .points
                .iter()
                .copied()
                .fold((5.0_f64, 0.0_f64), |acc, pt| {
                    if pt.1.abs() > acc.1.abs() { pt } else { acc }
                });
            assert!((apex.1.abs() - 5.0).abs() < 0.2, "arc apex {apex:?}");
        }
    }

    #[test]
    fn reads_old_style_polyline_with_vertices() {
        let body = "0\nPOLYLINE\n70\n1\n0\nVERTEX\n10\n0\n20\n0\n0\nVERTEX\n10\n10\n20\n0\n0\nVERTEX\n10\n10\n20\n10\n0\nSEQEND\n";
        let out = parse_dxf(&dxf(body));
        assert_eq!(out.len(), 1);
        for poly in &out {
            assert!(poly.closed);
            assert_eq!(poly.points.len(), 3);
        }
    }

    #[test]
    fn junk_lines_do_not_derail_the_read() {
        let out = parse_dxf("not a dxf\nat all\n0\nLINE\n10\n0\n20\n0\n11\n1\n21\n1\n");
        assert_eq!(out.len(), 1);
    }
}
