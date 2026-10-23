#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable,
    clippy::indexing_slicing
)]
//! A single-stroke stick font for laser text.
//!
//! Laser marking and engraving want single-line letters, not filled outlines, so
//! a galvo or diode traces each stroke once instead of flood-filling a glyph.
//! This module carries a clean-room stroke font for the printable ASCII a label,
//! a serial number or a date needs, and lays a string out into millimetre
//! polylines the engine then cuts.
//!
//! Glyphs live on a grid four units wide and seven tall, with the baseline at
//! `y = 0` and the cap line at `y = 7`. Lowercase maps to uppercase, the common
//! case for marking. An unknown character advances the pen without drawing.

/// A glyph is a list of strokes. Each stroke is a polyline in glyph-grid units.
type Glyph = &'static [&'static [(f64, f64)]];

/// The width of a glyph cell in grid units, before the inter-letter gap is added.
const GLYPH_WIDTH: f64 = 4.0;
/// The cap height in grid units. Height in millimetres scales against this.
const CAP_HEIGHT: f64 = 7.0;

/// Lay `text` out into millimetre polylines, left to right from `(origin_x, origin_y)`.
///
/// `height_mm` is the cap height. `gap_mm` is the extra space between glyph cells.
/// The baseline sits at `origin_y` and text grows upward. A newline returns to
/// `origin_x` and drops one line down, with a quarter-height line gap. Input is
/// capped at a few hundred characters so a bad caller cannot run away.
pub fn text_to_polylines(
    text: &str,
    height_mm: f64,
    origin_x: f64,
    origin_y: f64,
    gap_mm: f64,
) -> Vec<Vec<(f64, f64)>> {
    let scale = if height_mm > 0.0 {
        height_mm / CAP_HEIGHT
    } else {
        0.0
    };
    let advance = GLYPH_WIDTH * scale + gap_mm.max(0.0);
    let line_drop = height_mm + height_mm * 0.35;
    let mut out: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut pen_x = origin_x;
    let mut base_y = origin_y;
    for ch in text.chars().take(400) {
        if ch == '\n' {
            pen_x = origin_x;
            base_y -= line_drop;
            continue;
        }
        for stroke in glyph(ch) {
            let line: Vec<(f64, f64)> = stroke
                .iter()
                .map(|&(gx, gy)| (pen_x + gx * scale, base_y + gy * scale))
                .collect();
            if line.len() >= 2 {
                out.push(line);
            }
        }
        pen_x += advance;
    }
    out
}

/// The strokes for one character, uppercased. Unknown characters draw nothing.
fn glyph(c: char) -> Glyph {
    match c.to_ascii_uppercase() {
        ' ' => &[],
        'A' => &[
            &[(0.0, 0.0), (2.0, 7.0), (4.0, 0.0)],
            &[(1.0, 2.8), (3.0, 2.8)],
        ],
        'B' => &[
            &[
                (0.0, 0.0),
                (0.0, 7.0),
                (3.0, 7.0),
                (4.0, 6.0),
                (4.0, 4.5),
                (3.0, 3.5),
                (0.0, 3.5),
            ],
            &[
                (0.0, 3.5),
                (3.0, 3.5),
                (4.0, 2.5),
                (4.0, 1.0),
                (3.0, 0.0),
                (0.0, 0.0),
            ],
        ],
        'C' => &[&[
            (4.0, 5.5),
            (3.0, 7.0),
            (1.0, 7.0),
            (0.0, 5.5),
            (0.0, 1.5),
            (1.0, 0.0),
            (3.0, 0.0),
            (4.0, 1.5),
        ]],
        'D' => &[&[
            (0.0, 0.0),
            (0.0, 7.0),
            (2.5, 7.0),
            (4.0, 5.5),
            (4.0, 1.5),
            (2.5, 0.0),
            (0.0, 0.0),
        ]],
        'E' => &[
            &[(4.0, 7.0), (0.0, 7.0), (0.0, 0.0), (4.0, 0.0)],
            &[(0.0, 3.5), (2.8, 3.5)],
        ],
        'F' => &[
            &[(4.0, 7.0), (0.0, 7.0), (0.0, 0.0)],
            &[(0.0, 3.5), (2.8, 3.5)],
        ],
        'G' => &[&[
            (4.0, 5.5),
            (3.0, 7.0),
            (1.0, 7.0),
            (0.0, 5.5),
            (0.0, 1.5),
            (1.0, 0.0),
            (3.0, 0.0),
            (4.0, 1.5),
            (4.0, 3.0),
            (2.2, 3.0),
        ]],
        'H' => &[
            &[(0.0, 7.0), (0.0, 0.0)],
            &[(4.0, 7.0), (4.0, 0.0)],
            &[(0.0, 3.5), (4.0, 3.5)],
        ],
        'I' => &[
            &[(0.5, 7.0), (3.5, 7.0)],
            &[(2.0, 7.0), (2.0, 0.0)],
            &[(0.5, 0.0), (3.5, 0.0)],
        ],
        'J' => &[&[(4.0, 7.0), (4.0, 1.5), (3.0, 0.0), (1.0, 0.0), (0.0, 1.5)]],
        'K' => &[
            &[(0.0, 7.0), (0.0, 0.0)],
            &[(4.0, 7.0), (0.0, 3.2)],
            &[(1.4, 4.3), (4.0, 0.0)],
        ],
        'L' => &[&[(0.0, 7.0), (0.0, 0.0), (4.0, 0.0)]],
        'M' => &[&[(0.0, 0.0), (0.0, 7.0), (2.0, 3.5), (4.0, 7.0), (4.0, 0.0)]],
        'N' => &[&[(0.0, 0.0), (0.0, 7.0), (4.0, 0.0), (4.0, 7.0)]],
        'O' => &[&[
            (1.0, 0.0),
            (3.0, 0.0),
            (4.0, 1.5),
            (4.0, 5.5),
            (3.0, 7.0),
            (1.0, 7.0),
            (0.0, 5.5),
            (0.0, 1.5),
            (1.0, 0.0),
        ]],
        'P' => &[&[
            (0.0, 0.0),
            (0.0, 7.0),
            (3.0, 7.0),
            (4.0, 6.0),
            (4.0, 4.5),
            (3.0, 3.5),
            (0.0, 3.5),
        ]],
        'Q' => &[
            &[
                (1.0, 0.0),
                (3.0, 0.0),
                (4.0, 1.5),
                (4.0, 5.5),
                (3.0, 7.0),
                (1.0, 7.0),
                (0.0, 5.5),
                (0.0, 1.5),
                (1.0, 0.0),
            ],
            &[(2.5, 1.5), (4.0, -0.3)],
        ],
        'R' => &[
            &[
                (0.0, 0.0),
                (0.0, 7.0),
                (3.0, 7.0),
                (4.0, 6.0),
                (4.0, 4.5),
                (3.0, 3.5),
                (0.0, 3.5),
            ],
            &[(1.8, 3.5), (4.0, 0.0)],
        ],
        'S' => &[&[
            (4.0, 5.5),
            (3.0, 7.0),
            (1.0, 7.0),
            (0.0, 6.0),
            (0.0, 4.5),
            (1.0, 3.5),
            (3.0, 3.5),
            (4.0, 2.5),
            (4.0, 1.0),
            (3.0, 0.0),
            (1.0, 0.0),
            (0.0, 1.5),
        ]],
        'T' => &[&[(0.0, 7.0), (4.0, 7.0)], &[(2.0, 7.0), (2.0, 0.0)]],
        'U' => &[&[
            (0.0, 7.0),
            (0.0, 1.5),
            (1.0, 0.0),
            (3.0, 0.0),
            (4.0, 1.5),
            (4.0, 7.0),
        ]],
        'V' => &[&[(0.0, 7.0), (2.0, 0.0), (4.0, 7.0)]],
        'W' => &[&[(0.0, 7.0), (1.0, 0.0), (2.0, 3.5), (3.0, 0.0), (4.0, 7.0)]],
        'X' => &[&[(0.0, 7.0), (4.0, 0.0)], &[(0.0, 0.0), (4.0, 7.0)]],
        'Y' => &[
            &[(0.0, 7.0), (2.0, 3.5), (4.0, 7.0)],
            &[(2.0, 3.5), (2.0, 0.0)],
        ],
        'Z' => &[&[(0.0, 7.0), (4.0, 7.0), (0.0, 0.0), (4.0, 0.0)]],
        '0' => &[
            &[
                (1.0, 0.0),
                (3.0, 0.0),
                (4.0, 1.5),
                (4.0, 5.5),
                (3.0, 7.0),
                (1.0, 7.0),
                (0.0, 5.5),
                (0.0, 1.5),
                (1.0, 0.0),
            ],
            &[(0.5, 1.0), (3.5, 6.0)],
        ],
        '1' => &[
            &[(1.0, 5.5), (2.0, 7.0), (2.0, 0.0)],
            &[(0.8, 0.0), (3.2, 0.0)],
        ],
        '2' => &[&[
            (0.0, 5.5),
            (1.0, 7.0),
            (3.0, 7.0),
            (4.0, 5.5),
            (4.0, 4.5),
            (0.0, 0.0),
            (4.0, 0.0),
        ]],
        '3' => &[&[
            (0.0, 7.0),
            (4.0, 7.0),
            (2.0, 4.0),
            (3.0, 4.0),
            (4.0, 3.0),
            (4.0, 1.0),
            (3.0, 0.0),
            (1.0, 0.0),
            (0.0, 1.0),
        ]],
        '4' => &[&[(3.0, 0.0), (3.0, 7.0), (0.0, 2.5), (4.0, 2.5)]],
        '5' => &[&[
            (4.0, 7.0),
            (0.0, 7.0),
            (0.0, 4.0),
            (3.0, 4.0),
            (4.0, 3.0),
            (4.0, 1.0),
            (3.0, 0.0),
            (1.0, 0.0),
            (0.0, 1.0),
        ]],
        '6' => &[&[
            (4.0, 5.5),
            (3.0, 7.0),
            (1.0, 7.0),
            (0.0, 5.5),
            (0.0, 1.5),
            (1.0, 0.0),
            (3.0, 0.0),
            (4.0, 1.5),
            (4.0, 3.0),
            (3.0, 4.0),
            (1.0, 4.0),
            (0.0, 3.0),
        ]],
        '7' => &[&[(0.0, 7.0), (4.0, 7.0), (1.5, 0.0)]],
        '8' => &[&[
            (1.0, 3.5),
            (0.0, 4.5),
            (0.0, 6.0),
            (1.0, 7.0),
            (3.0, 7.0),
            (4.0, 6.0),
            (4.0, 4.5),
            (3.0, 3.5),
            (1.0, 3.5),
            (0.0, 2.5),
            (0.0, 1.0),
            (1.0, 0.0),
            (3.0, 0.0),
            (4.0, 1.0),
            (4.0, 2.5),
            (3.0, 3.5),
        ]],
        '9' => &[&[
            (0.0, 1.5),
            (1.0, 0.0),
            (3.0, 0.0),
            (4.0, 1.5),
            (4.0, 5.5),
            (3.0, 7.0),
            (1.0, 7.0),
            (0.0, 5.5),
            (0.0, 4.0),
            (1.0, 3.0),
            (3.0, 3.0),
            (4.0, 4.0),
        ]],
        '.' => &[&[(1.8, 0.0), (1.8, 0.4)]],
        ',' => &[&[(2.2, 0.5), (1.6, -0.6)]],
        '-' => &[&[(0.8, 3.5), (3.2, 3.5)]],
        '_' => &[&[(0.0, -0.3), (4.0, -0.3)]],
        ':' => &[&[(2.0, 4.6), (2.0, 5.0)], &[(2.0, 1.6), (2.0, 2.0)]],
        ';' => &[&[(2.0, 4.6), (2.0, 5.0)], &[(2.2, 1.0), (1.6, 0.0)]],
        '/' => &[&[(0.0, 0.0), (4.0, 7.0)]],
        '\\' => &[&[(0.0, 7.0), (4.0, 0.0)]],
        '+' => &[&[(2.0, 1.5), (2.0, 5.5)], &[(0.0, 3.5), (4.0, 3.5)]],
        '=' => &[&[(0.5, 4.5), (3.5, 4.5)], &[(0.5, 2.5), (3.5, 2.5)]],
        '*' => &[
            &[(2.0, 4.0), (2.0, 7.0)],
            &[(0.7, 4.6), (3.3, 6.4)],
            &[(3.3, 4.6), (0.7, 6.4)],
        ],
        '#' => &[
            &[(1.2, 0.0), (1.8, 7.0)],
            &[(2.6, 0.0), (3.2, 7.0)],
            &[(0.3, 2.3), (4.3, 2.3)],
            &[(0.1, 4.7), (4.1, 4.7)],
        ],
        '(' => &[&[(3.0, 7.0), (1.5, 5.5), (1.5, 1.5), (3.0, 0.0)]],
        ')' => &[&[(1.0, 7.0), (2.5, 5.5), (2.5, 1.5), (1.0, 0.0)]],
        '[' => &[&[(3.0, 7.0), (1.5, 7.0), (1.5, 0.0), (3.0, 0.0)]],
        ']' => &[&[(1.0, 7.0), (2.5, 7.0), (2.5, 0.0), (1.0, 0.0)]],
        '!' => &[&[(2.0, 7.0), (2.0, 2.0)], &[(2.0, 0.0), (2.0, 0.4)]],
        '?' => &[
            &[
                (0.0, 5.5),
                (1.0, 7.0),
                (3.0, 7.0),
                (4.0, 5.5),
                (4.0, 4.5),
                (2.0, 3.0),
                (2.0, 2.0),
            ],
            &[(2.0, 0.0), (2.0, 0.4)],
        ],
        '%' => &[
            &[(0.0, 0.0), (4.0, 7.0)],
            &[(0.3, 5.2), (1.5, 5.2), (1.5, 6.8), (0.3, 6.8), (0.3, 5.2)],
            &[(2.5, 0.2), (3.7, 0.2), (3.7, 1.8), (2.5, 1.8), (2.5, 0.2)],
        ],
        '"' => &[&[(1.3, 7.0), (1.3, 5.5)], &[(2.7, 7.0), (2.7, 5.5)]],
        '\'' => &[&[(2.0, 7.0), (2.0, 5.5)]],
        _ => &[],
    }
}

/// Collects a glyph's outline into flattened, closed contours in millimetres.
struct Outliner {
    contours: Vec<Vec<(f64, f64)>>,
    current: Vec<(f64, f64)>,
    last: (f64, f64),
    scale: f64,
    ox: f64,
    oy: f64,
}

impl Outliner {
    fn at(&self, x: f32, y: f32) -> (f64, f64) {
        (
            self.ox + f64::from(x) * self.scale,
            self.oy + f64::from(y) * self.scale,
        )
    }
    fn flush(&mut self) {
        if self.current.len() >= 3 {
            self.contours.push(core::mem::take(&mut self.current));
        } else {
            self.current.clear();
        }
    }
}

impl skrifa::outline::OutlinePen for Outliner {
    fn move_to(&mut self, x: f32, y: f32) {
        self.flush();
        let p = self.at(x, y);
        self.current.push(p);
        self.last = p;
    }
    fn line_to(&mut self, x: f32, y: f32) {
        let p = self.at(x, y);
        self.current.push(p);
        self.last = p;
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let (c, e, s) = (self.at(x1, y1), self.at(x, y), self.last);
        for i in 1..=8 {
            let t = f64::from(i) / 8.0;
            let u = 1.0 - t;
            self.current.push((
                u * u * s.0 + 2.0 * u * t * c.0 + t * t * e.0,
                u * u * s.1 + 2.0 * u * t * c.1 + t * t * e.1,
            ));
        }
        self.last = e;
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let (c1, c2, e, s) = (self.at(x1, y1), self.at(x2, y2), self.at(x, y), self.last);
        for i in 1..=10 {
            let t = f64::from(i) / 10.0;
            let u = 1.0 - t;
            let w0 = u * u * u;
            let w1 = 3.0 * u * u * t;
            let w2 = 3.0 * u * t * t;
            let w3 = t * t * t;
            self.current.push((
                w0 * s.0 + w1 * c1.0 + w2 * c2.0 + w3 * e.0,
                w0 * s.1 + w1 * c1.1 + w2 * c2.1 + w3 * e.1,
            ));
        }
        self.last = e;
    }
    fn close(&mut self) {
        self.flush();
    }
}

/// Lay `text` out into filled glyph outlines from a TrueType or OpenType font.
///
/// `font` is the raw font file. `height_mm` is the cap height, so filled text and
/// the single-stroke font share a scale. Each contour is a closed polyline in
/// millimetres. The font loads at runtime from a served asset, so the engine
/// carries no embedded font and the build stays small. A glyph with no outline,
/// like a space, only advances the pen.
pub fn glyph_outlines(
    font: &[u8],
    text: &str,
    height_mm: f64,
    origin_x: f64,
    origin_y: f64,
) -> Vec<Vec<(f64, f64)>> {
    use skrifa::MetadataProvider;
    use skrifa::instance::{LocationRef, Size};
    let Ok(face) = skrifa::FontRef::new(font) else {
        return Vec::new();
    };
    // Unscaled metrics report font units, which the outliner then scales to mm.
    let metrics = face.metrics(Size::unscaled(), LocationRef::default());
    let upem = f64::from(metrics.units_per_em);
    if upem <= 0.0 {
        return Vec::new();
    }
    // Scale so the font's cap height matches the requested height, matching the
    // stick font. Fall back to a common cap fraction of the em when unknown.
    let cap = metrics
        .cap_height
        .filter(|&c| c > 0.0)
        .map(f64::from)
        .unwrap_or(upem * 0.7);
    let scale = if cap > 0.0 { height_mm / cap } else { 0.0 };
    let line_drop = height_mm + height_mm * 0.35;
    let outlines = face.outline_glyphs();
    let glyph_metrics = face.glyph_metrics(Size::unscaled(), LocationRef::default());
    let charmap = face.charmap();
    let mut out: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut pen_x = origin_x;
    let mut base_y = origin_y;
    for ch in text.chars().take(400) {
        if ch == '\n' {
            pen_x = origin_x;
            base_y -= line_drop;
            continue;
        }
        let Some(gid) = charmap.map(ch) else {
            pen_x += upem * 0.5 * scale;
            continue;
        };
        if let Some(glyph) = outlines.get(gid) {
            let mut outliner = Outliner {
                contours: Vec::new(),
                current: Vec::new(),
                last: (0.0, 0.0),
                scale,
                ox: pen_x,
                oy: base_y,
            };
            let draw =
                skrifa::outline::DrawSettings::unhinted(Size::unscaled(), LocationRef::default());
            let _ = glyph.draw(draw, &mut outliner);
            outliner.flush();
            out.extend(outliner.contours);
        }
        let advance = glyph_metrics
            .advance_width(gid)
            .map(f64::from)
            .unwrap_or(upem * 0.5);
        pen_x += advance * scale;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_text_draws_nothing() {
        assert!(text_to_polylines("", 10.0, 0.0, 0.0, 1.0).is_empty());
        assert!(text_to_polylines("   ", 10.0, 0.0, 0.0, 1.0).is_empty());
    }

    #[test]
    fn height_scales_the_cap() {
        // An uppercase I is a stem from the baseline to the cap line.
        let out = text_to_polylines("I", 14.0, 0.0, 0.0, 0.0);
        let top = out.iter().flatten().fold(0.0_f64, |m, &(_, y)| m.max(y));
        assert!((top - 14.0).abs() < 0.01, "cap top {top}");
    }

    #[test]
    fn letters_advance_left_to_right() {
        let out = text_to_polylines("AB", 7.0, 0.0, 0.0, 1.0);
        // The second glyph sits a full cell to the right of the first.
        let max_x = out.iter().flatten().fold(0.0_f64, |m, &(x, _)| m.max(x));
        assert!(max_x > 5.0, "run width {max_x}");
    }

    #[test]
    fn newline_drops_a_line() {
        let out = text_to_polylines("A\nA", 7.0, 0.0, 0.0, 1.0);
        let min_y = out.iter().flatten().fold(0.0_f64, |m, &(_, y)| m.min(y));
        assert!(min_y < -1.0, "second line baseline {min_y}");
    }

    #[test]
    fn lowercase_maps_to_uppercase() {
        let lower = text_to_polylines("a", 7.0, 0.0, 0.0, 0.0);
        let upper = text_to_polylines("A", 7.0, 0.0, 0.0, 0.0);
        assert_eq!(lower.len(), upper.len());
    }

    #[test]
    fn glyph_outlines_reject_bad_font_bytes() {
        // A non-font buffer must return empty, never panic.
        assert!(glyph_outlines(b"not a font at all", "ABC", 10.0, 0.0, 0.0).is_empty());
        assert!(glyph_outlines(&[], "A", 10.0, 0.0, 0.0).is_empty());
    }

    #[test]
    fn unknown_character_only_advances() {
        // A control character draws nothing but still moves the pen for the next glyph.
        let out = text_to_polylines("\u{7}A", 7.0, 0.0, 0.0, 1.0);
        let min_x = out.iter().flatten().fold(f64::MAX, |m, &(x, _)| m.min(x));
        assert!(min_x > 4.0, "second glyph start {min_x}");
    }
}
