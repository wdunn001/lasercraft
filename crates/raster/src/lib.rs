#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! Raster image engraving for LaserCraft.
//!
//! Image engraving is LightBurn's core workflow: a grayscale image becomes
//! power-modulated horizontal scan lines. See `docs/rea-findings.md` for the
//! `acclplanner` image-fill functions. This crate converts a [`GrayImage`] into
//! [`ScanRun`] spans of constant power, which the engine emits as cutting moves.
//! Adjacent pixels of equal power merge into one run so the output stays compact.
//! Boustrophedon travel ordering is a later optimization.

/// A grayscale image. A higher value means a darker pixel and more laser power.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrayImage {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// Row-major samples, `width * height` of them, 0 to 255.
    pub pixels: Vec<u8>,
}

impl GrayImage {
    /// Read a sample, returning 0 for out-of-bounds coordinates.
    pub fn get(&self, x: u32, y: u32) -> u8 {
        if x >= self.width || y >= self.height {
            return 0;
        }
        let index = (y as usize) * (self.width as usize) + (x as usize);
        self.pixels.get(index).copied().unwrap_or(0)
    }
}

/// Parameters for a raster scan.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RasterParams {
    /// The physical size of one pixel in millimetres.
    pub pixel_mm: f64,
    /// The power percentage mapped to a fully dark pixel.
    pub power_max: f64,
    /// Samples at or below this value are skipped as blank.
    pub threshold: u8,
    /// Travel speed in millimetres per second for the engrave moves.
    pub speed_mm_s: f64,
}

/// One horizontal span burned at a constant power.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScanRun {
    /// The row position in millimetres.
    pub y_mm: f64,
    /// Where the span starts in millimetres.
    pub x_start_mm: f64,
    /// Where the span ends in millimetres.
    pub x_end_mm: f64,
    /// The power percentage for the span.
    pub power: f64,
}

/// Convert an image into constant-power horizontal scan runs.
pub fn raster_scan(image: &GrayImage, params: &RasterParams) -> Vec<ScanRun> {
    let mut runs = Vec::new();
    if params.pixel_mm <= 0.0 {
        return runs;
    }
    for y in 0..image.height {
        let y_mm = f64::from(y) * params.pixel_mm;
        let mut x = 0u32;
        while x < image.width {
            let power = power_percent(image.get(x, y), params);
            if power == 0 {
                x += 1;
                continue;
            }
            let start = x;
            while x < image.width && power_percent(image.get(x, y), params) == power {
                x += 1;
            }
            runs.push(ScanRun {
                y_mm,
                x_start_mm: f64::from(start) * params.pixel_mm,
                x_end_mm: f64::from(x) * params.pixel_mm,
                power: f64::from(power),
            });
        }
    }
    runs
}

/// The quantized power percent for one sample, 0 when below the threshold.
fn power_percent(sample: u8, params: &RasterParams) -> u8 {
    if sample <= params.threshold {
        return 0;
    }
    let scaled = f64::from(sample) / 255.0 * params.power_max;
    scaled.round().clamp(0.0, 100.0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params() -> RasterParams {
        RasterParams {
            pixel_mm: 1.0,
            power_max: 100.0,
            threshold: 128,
            speed_mm_s: 150.0,
        }
    }

    #[test]
    fn merges_equal_power_pixels_into_one_run() {
        // One blank pixel then two dark pixels of equal value.
        let image = GrayImage {
            width: 3,
            height: 1,
            pixels: vec![0, 200, 200],
        };
        let runs = raster_scan(&image, &params());
        assert_eq!(runs.len(), 1);
        let run = runs[0];
        assert_eq!(run.x_start_mm, 1.0);
        assert_eq!(run.x_end_mm, 3.0);
        // 200/255 * 100 rounds to 78 percent.
        assert_eq!(run.power, 78.0);
    }

    #[test]
    fn blank_image_yields_no_runs() {
        let image = GrayImage {
            width: 4,
            height: 2,
            pixels: vec![0, 10, 128, 100, 0, 0, 0, 0],
        };
        // All samples are at or below the 128 threshold.
        assert!(raster_scan(&image, &params()).is_empty());
    }
}
