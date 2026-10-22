#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! Galvo and fiber laser support for LaserCraft.
//!
//! Reverse engineering of `executor.dll` found a `CalibFunc` class doing galvo
//! field correction, with `logicPtToDevPt` mapping a logical point in the field to
//! device coordinates, a `GalvoParam` carrying the field, and `setResolution`. See
//! `docs/rea-findings.md`. This crate reproduces that mapping. Grid correction from
//! a measured calibration file lands on top of it later.

use lasercraft_device::{Descriptor, Protocol, TransportKind};

/// The galvo field parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GalvoParam {
    /// The square field side length in millimetres.
    pub field_size_mm: f64,
    /// The device coordinate resolution in bits, as set by `setResolution`.
    pub resolution_bits: u32,
}

impl GalvoParam {
    /// The maximum device coordinate value for the configured resolution.
    pub fn device_max(&self) -> u32 {
        let bits = self.resolution_bits.clamp(1, 31);
        (1u32 << bits) - 1
    }

    /// Map a logical point in millimetres to device coordinates.
    ///
    /// The logical range `[-field/2, +field/2]` maps linearly onto
    /// `[0, device_max]`. Points outside the field are clamped. This is the
    /// `logicPtToDevPt` behaviour recovered from `executor.dll`.
    pub fn logic_to_device(&self, x_mm: f64, y_mm: f64) -> (i32, i32) {
        (self.axis_to_device(x_mm), self.axis_to_device(y_mm))
    }

    fn axis_to_device(&self, mm: f64) -> i32 {
        let half = self.field_size_mm / 2.0;
        let span = self.field_size_mm.max(f64::MIN_POSITIVE);
        let max = f64::from(self.device_max());
        let normalized = ((mm + half) / span).clamp(0.0, 1.0);
        (normalized * max).round() as i32
    }
}

/// Build the descriptor for a galvo controller.
pub fn galvo_descriptor() -> Descriptor {
    Descriptor {
        id: "galvo".to_string(),
        display_name: "Galvo / Fiber".to_string(),
        transport: TransportKind::Usb,
        gcode_flavor: None,
    }
}

/// A galvo controller protocol instance.
#[derive(Debug, Clone, PartialEq)]
pub struct GalvoProtocol {
    param: GalvoParam,
    descriptor: Descriptor,
}

impl GalvoProtocol {
    /// A galvo protocol with the given field parameters.
    pub fn new(param: GalvoParam) -> Self {
        Self {
            param,
            descriptor: galvo_descriptor(),
        }
    }

    /// The field parameters.
    pub fn param(&self) -> GalvoParam {
        self.param
    }
}

impl Protocol for GalvoProtocol {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

/// One galvo marking instruction in device coordinates.
///
/// A galvo controller does not speak GCode. It consumes a list of jumps and marks
/// over the mirror field, with power and the MOPA pulse frequency set between them.
/// This is the device-neutral marking model. The board-specific byte encoding
/// (the JCZ or LMC list format behind EZCAD2) serializes these later, the same way
/// a GCode dialect serializes the neutral ops.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GalvoCommand {
    /// Move with the laser off to a device point.
    Jump {
        /// Device X.
        x: i32,
        /// Device Y.
        y: i32,
    },
    /// Move with the laser on to a device point.
    Mark {
        /// Device X.
        x: i32,
        /// Device Y.
        y: i32,
    },
    /// Set the marking power as a percentage.
    SetPower {
        /// Power percent from 0.0 to 100.0.
        percent: f64,
    },
    /// Set the MOPA pulse frequency in hertz.
    SetFrequency {
        /// Frequency in hertz.
        hz: u32,
    },
}

/// Convert a logical polyline in millimetres into galvo marking commands.
///
/// Power and frequency are set first, then a jump to the first point, then a mark
/// to each subsequent point. A closed polyline marks back to its start. Points are
/// mapped through the field correction in `param`. Fewer than two points yields
/// only the power and frequency commands.
pub fn mark_polyline(
    param: &GalvoParam,
    points: &[(f64, f64)],
    closed: bool,
    power_percent: f64,
    frequency_hz: u32,
) -> Vec<GalvoCommand> {
    let mut commands = vec![
        GalvoCommand::SetPower {
            percent: power_percent.clamp(0.0, 100.0),
        },
        GalvoCommand::SetFrequency { hz: frequency_hz },
    ];
    let Some((first_x, first_y)) = points.first().copied() else {
        return commands;
    };
    if points.len() < 2 {
        return commands;
    }
    let (jx, jy) = param.logic_to_device(first_x, first_y);
    commands.push(GalvoCommand::Jump { x: jx, y: jy });
    for &(x, y) in &points[1..] {
        let (dx, dy) = param.logic_to_device(x, y);
        commands.push(GalvoCommand::Mark { x: dx, y: dy });
    }
    if closed {
        commands.push(GalvoCommand::Mark { x: jx, y: jy });
    }
    commands
}

/// One constant-power horizontal span to mark, in millimetres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MarkRun {
    /// The row position.
    pub y_mm: f64,
    /// Where the span starts.
    pub x_start_mm: f64,
    /// Where the span ends.
    pub x_end_mm: f64,
    /// The marking power percentage.
    pub power_percent: f64,
}

/// Convert constant-power scan runs into galvo marking commands.
///
/// This is the galvo analogue of raster engraving: each run jumps to its start
/// with the beam off, then marks across at the run power. Power is set only when it
/// changes between runs, so a flat image does not re-send it on every line.
pub fn mark_scanlines(
    param: &GalvoParam,
    runs: &[MarkRun],
    frequency_hz: u32,
) -> Vec<GalvoCommand> {
    let mut commands = vec![GalvoCommand::SetFrequency { hz: frequency_hz }];
    let mut last_power: Option<i64> = None;
    for run in runs {
        let quantized = run.power_percent.round() as i64;
        if last_power != Some(quantized) {
            commands.push(GalvoCommand::SetPower {
                percent: run.power_percent.clamp(0.0, 100.0),
            });
            last_power = Some(quantized);
        }
        let (sx, sy) = param.logic_to_device(run.x_start_mm, run.y_mm);
        let (ex, ey) = param.logic_to_device(run.x_end_mm, run.y_mm);
        commands.push(GalvoCommand::Jump { x: sx, y: sy });
        commands.push(GalvoCommand::Mark { x: ex, y: ey });
    }
    commands
}

/// A galvo lens correction table parsed from an EZCAD `JCZ_COR_2_1` `.cor` file.
///
/// The file is a signature, a short header carrying a scale float, then a square
/// grid of per-cell `(dx, dy)` device-unit offsets that straighten the lens
/// distortion across the field. Sampling bilinearly between cells gives the
/// correction to add at any field position. Format confirmed against a real
/// ComMarker B6 `jcz15.cor` (65x65 grid, 40-byte header).
#[derive(Debug, Clone, PartialEq)]
pub struct CorrectionTable {
    /// The grid side length, for example 65.
    pub size: usize,
    /// The scale factor from the header.
    pub scale: f32,
    cells: Vec<(i32, i32)>,
}

const COR_SIGNATURE: &[u8] = b"JCZ_COR_2_1";
const COR_HEADER_LEN: usize = 40;

impl CorrectionTable {
    /// Parse a `.cor` file. Errors on a bad signature or a non-square grid.
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < COR_HEADER_LEN {
            return Err("cor file too short");
        }
        if bytes.get(..COR_SIGNATURE.len()) != Some(COR_SIGNATURE) {
            return Err("not a JCZ_COR correction file");
        }
        let data_len = bytes.len() - COR_HEADER_LEN;
        if !data_len.is_multiple_of(8) {
            return Err("cor data is not whole (dx, dy) pairs");
        }
        let points = data_len / 8;
        let size = (points as f64).sqrt().round() as usize;
        if size * size != points {
            return Err("cor grid is not square");
        }
        let scale_bytes: [u8; 4] = bytes
            .get(32..36)
            .and_then(|s| s.try_into().ok())
            .ok_or("cor header truncated")?;
        let scale = f32::from_le_bytes(scale_bytes);
        let mut cells = Vec::with_capacity(points);
        for i in 0..points {
            let o = COR_HEADER_LEN + i * 8;
            let dx = read_i32(bytes, o)?;
            let dy = read_i32(bytes, o + 4)?;
            cells.push((dx, dy));
        }
        Ok(Self { size, scale, cells })
    }

    fn cell(&self, gx: usize, gy: usize) -> (i32, i32) {
        self.cells
            .get(gy * self.size + gx)
            .copied()
            .unwrap_or((0, 0))
    }

    /// Bilinearly sample the `(dx, dy)` correction for normalized field coordinates,
    /// where `u` and `v` run from 0.0 to 1.0 across the field. Out-of-range inputs
    /// clamp to the edge.
    pub fn correction_at(&self, u: f64, v: f64) -> (f64, f64) {
        if self.size < 2 {
            return (0.0, 0.0);
        }
        let max_index = (self.size - 1) as f64;
        let fx = u.clamp(0.0, 1.0) * max_index;
        let fy = v.clamp(0.0, 1.0) * max_index;
        let x0 = fx.floor() as usize;
        let y0 = fy.floor() as usize;
        let x1 = (x0 + 1).min(self.size - 1);
        let y1 = (y0 + 1).min(self.size - 1);
        let tx = fx - x0 as f64;
        let ty = fy - y0 as f64;
        let lerp = |a: f64, b: f64, t: f64| a + (b - a) * t;
        let (c00, c10, c01, c11) = (
            self.cell(x0, y0),
            self.cell(x1, y0),
            self.cell(x0, y1),
            self.cell(x1, y1),
        );
        let dx = lerp(
            lerp(f64::from(c00.0), f64::from(c10.0), tx),
            lerp(f64::from(c01.0), f64::from(c11.0), tx),
            ty,
        );
        let dy = lerp(
            lerp(f64::from(c00.1), f64::from(c10.1), tx),
            lerp(f64::from(c01.1), f64::from(c11.1), tx),
            ty,
        );
        (dx, dy)
    }
}

fn read_i32(bytes: &[u8], offset: usize) -> Result<i32, &'static str> {
    bytes
        .get(offset..offset + 4)
        .and_then(|s| s.try_into().ok())
        .map(i32::from_le_bytes)
        .ok_or("cor data truncated")
}

#[cfg(test)]
mod cor_tests {
    use super::*;

    fn synthetic_cor() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(b"JCZ_COR_2_1\0");
        b.resize(32, 0);
        b.extend_from_slice(&1.0f32.to_le_bytes());
        b.resize(COR_HEADER_LEN, 0);
        // A 2x2 grid of corrections.
        for (dx, dy) in [(0i32, 0i32), (100, 0), (0, 100), (100, 100)] {
            b.extend_from_slice(&dx.to_le_bytes());
            b.extend_from_slice(&dy.to_le_bytes());
        }
        b
    }

    #[test]
    fn parses_and_samples_a_cor_grid() {
        let table = CorrectionTable::parse(&synthetic_cor());
        assert!(table.is_ok());
        if let Ok(table) = table {
            assert_eq!(table.size, 2);
            assert_eq!(table.scale, 1.0);
            // Corners read the grid cells directly.
            assert_eq!(table.correction_at(0.0, 0.0), (0.0, 0.0));
            assert_eq!(table.correction_at(1.0, 1.0), (100.0, 100.0));
            // The centre bilinearly averages to (50, 50).
            assert_eq!(table.correction_at(0.5, 0.5), (50.0, 50.0));
        }
    }

    #[test]
    fn rejects_a_non_cor_file() {
        assert!(CorrectionTable::parse(b"not a cor file at all......").is_err());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanlines_set_power_once_per_change() {
        let param = GalvoParam {
            field_size_mm: 100.0,
            resolution_bits: 16,
        };
        let runs = [
            MarkRun {
                y_mm: 0.0,
                x_start_mm: 0.0,
                x_end_mm: 10.0,
                power_percent: 60.0,
            },
            MarkRun {
                y_mm: 1.0,
                x_start_mm: 0.0,
                x_end_mm: 10.0,
                power_percent: 60.0,
            },
        ];
        let commands = mark_scanlines(&param, &runs, 30000);
        // Frequency, one power (shared), then jump+mark per run.
        let powers = commands
            .iter()
            .filter(|c| matches!(c, GalvoCommand::SetPower { .. }))
            .count();
        assert_eq!(powers, 1);
        let marks = commands
            .iter()
            .filter(|c| matches!(c, GalvoCommand::Mark { .. }))
            .count();
        assert_eq!(marks, 2);
    }

    #[test]
    fn field_center_maps_to_mid_scale() {
        let param = GalvoParam {
            field_size_mm: 100.0,
            resolution_bits: 16,
        };
        assert_eq!(param.device_max(), 65535);
        let (x, y) = param.logic_to_device(0.0, 0.0);
        assert_eq!(x, 32768);
        assert_eq!(y, 32768);
    }

    #[test]
    fn mark_polyline_jumps_then_marks_and_closes() {
        let param = GalvoParam {
            field_size_mm: 100.0,
            resolution_bits: 16,
        };
        // A triangle centred near the field centre.
        let points = [(0.0, 0.0), (10.0, 0.0), (5.0, 8.0)];
        let commands = mark_polyline(&param, &points, true, 70.0, 20000);
        // Power, frequency, one jump, two marks along, one closing mark.
        assert_eq!(commands.len(), 6);
        assert_eq!(commands[0], GalvoCommand::SetPower { percent: 70.0 });
        assert_eq!(commands[1], GalvoCommand::SetFrequency { hz: 20000 });
        assert!(matches!(commands[2], GalvoCommand::Jump { .. }));
        assert!(matches!(commands[5], GalvoCommand::Mark { .. }));
        // The closing mark returns to the first point's device coordinate.
        let (fx, fy) = param.logic_to_device(0.0, 0.0);
        assert_eq!(commands[5], GalvoCommand::Mark { x: fx, y: fy });
    }

    #[test]
    fn field_edges_clamp_to_range() {
        let param = GalvoParam {
            field_size_mm: 100.0,
            resolution_bits: 16,
        };
        assert_eq!(param.logic_to_device(-50.0, 50.0), (0, 65535));
        assert_eq!(param.logic_to_device(-999.0, 999.0), (0, 65535));
    }
}
