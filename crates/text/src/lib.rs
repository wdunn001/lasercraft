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
    fn unknown_character_only_advances() {
        // A control character draws nothing but still moves the pen for the next glyph.
        let out = text_to_polylines("\u{7}A", 7.0, 0.0, 0.0, 1.0);
        let min_x = out.iter().flatten().fold(f64::MAX, |m, &(x, _)| m.min(x));
        assert!(min_x > 4.0, "second glyph start {min_x}");
    }
}
