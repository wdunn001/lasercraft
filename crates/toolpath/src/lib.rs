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

#[cfg(test)]
mod tests {
    use super::*;

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
}
