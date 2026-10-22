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
}
