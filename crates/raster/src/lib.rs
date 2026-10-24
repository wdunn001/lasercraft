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
//! Image engraving is the core workflow: a grayscale image becomes
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

    /// Parse a binary PGM (`P5`) image.
    ///
    /// PGM is the simplest lossless grayscale container, so it is the first import
    /// format. The header may carry `#` comments. A maxval other than 255 is scaled
    /// to the full range. Returns a borrowed static error message on malformed input
    /// rather than panicking.
    pub fn from_pgm(bytes: &[u8]) -> Result<GrayImage, &'static str> {
        let mut pos = 0usize;
        let magic = read_token(bytes, &mut pos).ok_or("missing PGM magic")?;
        if magic != b"P5" {
            return Err("not a binary PGM (expected P5)");
        }
        let width = read_uint(bytes, &mut pos)?;
        let height = read_uint(bytes, &mut pos)?;
        let maxval = read_uint(bytes, &mut pos)?;
        if maxval == 0 || maxval > 255 {
            return Err("unsupported PGM maxval");
        }
        // Exactly one whitespace byte separates the header from the pixel data.
        match bytes.get(pos) {
            Some(b) if is_ws(*b) => pos += 1,
            _ => return Err("missing PGM data separator"),
        }
        let count = (width as usize)
            .checked_mul(height as usize)
            .ok_or("PGM dimensions overflow")?;
        // checked_add guards the 32-bit usize on wasm32, where pos + count could wrap.
        let end = pos.checked_add(count).ok_or("PGM pixel offset overflow")?;
        let data = bytes.get(pos..end).ok_or("truncated PGM pixel data")?;
        let pixels = if maxval == 255 {
            data.to_vec()
        } else {
            data.iter()
                .map(|&b| ((u32::from(b) * 255) / maxval) as u8)
                .collect()
        };
        Ok(GrayImage {
            width,
            height,
            pixels,
        })
    }
}

fn is_ws(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0c)
}

fn skip_ws_and_comments(bytes: &[u8], pos: &mut usize) {
    while let Some(&b) = bytes.get(*pos) {
        if is_ws(b) {
            *pos += 1;
        } else if b == b'#' {
            while let Some(&c) = bytes.get(*pos) {
                *pos += 1;
                if c == b'\n' {
                    break;
                }
            }
        } else {
            break;
        }
    }
}

fn read_token<'a>(bytes: &'a [u8], pos: &mut usize) -> Option<&'a [u8]> {
    skip_ws_and_comments(bytes, pos);
    let start = *pos;
    while let Some(&b) = bytes.get(*pos) {
        if is_ws(b) {
            break;
        }
        *pos += 1;
    }
    if *pos == start {
        None
    } else {
        bytes.get(start..*pos)
    }
}

fn read_uint(bytes: &[u8], pos: &mut usize) -> Result<u32, &'static str> {
    let token = read_token(bytes, pos).ok_or("missing PGM header number")?;
    let text = std::str::from_utf8(token).map_err(|_| "non-ascii PGM header")?;
    text.parse::<u32>().map_err(|_| "invalid PGM header number")
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
        let mut row_runs = Vec::new();
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
            row_runs.push(ScanRun {
                y_mm,
                x_start_mm: f64::from(start) * params.pixel_mm,
                x_end_mm: f64::from(x) * params.pixel_mm,
                power: f64::from(power),
            });
        }
        // Boustrophedon: odd rows sweep right to left so the head does not fly
        // back. Reverse the run order and the direction of each run.
        if y % 2 == 1 {
            row_runs.reverse();
            for run in &mut row_runs {
                std::mem::swap(&mut run.x_start_mm, &mut run.x_end_mm);
            }
        }
        runs.append(&mut row_runs);
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
    fn odd_rows_sweep_in_reverse() {
        let image = GrayImage {
            width: 2,
            height: 2,
            pixels: vec![255, 255, 255, 255],
        };
        let runs = raster_scan(&image, &params());
        assert_eq!(runs.len(), 2);
        // Row 0 runs left to right.
        assert!(runs[0].x_start_mm < runs[0].x_end_mm);
        assert_eq!(runs[0].y_mm, 0.0);
        // Row 1 runs right to left.
        assert!(runs[1].x_start_mm > runs[1].x_end_mm);
        assert_eq!(runs[1].y_mm, 1.0);
    }

    #[test]
    fn parses_a_binary_pgm_with_a_comment() {
        let mut bytes = b"P5\n# a comment\n2 1\n255\n".to_vec();
        bytes.push(0);
        bytes.push(200);
        let parsed = GrayImage::from_pgm(&bytes);
        assert!(parsed.is_ok(), "a valid PGM should parse");
        if let Ok(image) = parsed {
            assert_eq!(image.width, 2);
            assert_eq!(image.height, 1);
            assert_eq!(image.pixels, vec![0, 200]);
        }
    }

    #[test]
    fn rejects_truncated_pgm() {
        // Header promises two pixels but only one byte follows.
        let bytes = b"P5\n2 1\n255\n\x10".to_vec();
        assert!(GrayImage::from_pgm(&bytes).is_err());
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
