#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! GCode emission for LaserCraft.
//!
//! Reverse engineering showed LightBurn's GCode controllers share one base with a
//! subclass per dialect (GRBL and its variants, Marlin, Smoothie, RepRap,
//! LinuxCNC, Klipper, and vendor flavors). See `docs/rea-findings.md`. This crate
//! follows the same shape: a core set of device-neutral [`Op`] values and a
//! [`Dialect`] trait that each controller implements. A dialect is a small
//! implementation, never a copy of the whole emitter.

use std::fmt::Write as _;

/// The GCode flavor a dialect speaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flavor {
    /// GRBL and compatible controllers.
    Grbl,
    /// Marlin firmware.
    Marlin,
    /// Smoothieware.
    Smoothie,
    /// RepRapFirmware.
    RepRap,
    /// LinuxCNC.
    LinuxCnc,
    /// Klipper, as emitted by `acclplanner.dll`.
    Klipper,
}

/// A device-neutral motion or laser operation.
#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    /// Rapid travel with the laser off.
    RapidTo {
        /// Target X in millimetres.
        x: f64,
        /// Target Y in millimetres.
        y: f64,
    },
    /// Cutting move at a feed rate and power.
    CutTo {
        /// Target X in millimetres.
        x: f64,
        /// Target Y in millimetres.
        y: f64,
        /// Feed rate in millimetres per second.
        feed_mm_s: f64,
        /// Laser power as a percentage from 0.0 to 100.0.
        power: f64,
    },
    /// Turn the laser off.
    LaserOff,
}

/// A controller dialect. One implementation per GCode family.
pub trait Dialect {
    /// The flavor this dialect speaks.
    fn flavor(&self) -> Flavor;
    /// The lines that open a program.
    fn header(&self) -> String;
    /// The lines that close a program.
    fn footer(&self) -> String;
    /// One operation rendered as a GCode line without a trailing newline.
    fn emit(&self, op: &Op) -> String;
}

/// Render a whole program: header, every op, then footer.
pub fn program(dialect: &dyn Dialect, ops: &[Op]) -> String {
    let mut out = String::new();
    out.push_str(&dialect.header());
    for op in ops {
        let line = dialect.emit(op);
        // `write!` into a String cannot fail; discard the Result to satisfy lints.
        let _ = writeln!(out, "{line}");
    }
    out.push_str(&dialect.footer());
    out
}

/// The GRBL dialect, the most common controller family.
#[derive(Debug, Default, Clone, Copy)]
pub struct Grbl {
    /// The S value that maps to 100 percent power, set by the `$30` max spindle value.
    pub spindle_max: u32,
}

impl Grbl {
    /// A GRBL dialect with the stock `$30` value of 1000.
    pub fn new() -> Self {
        Self { spindle_max: 1000 }
    }

    fn spindle_for(&self, power: f64) -> u32 {
        let clamped = power.clamp(0.0, 100.0);
        let scaled = clamped / 100.0 * f64::from(self.spindle_max);
        scaled.round() as u32
    }
}

impl Dialect for Grbl {
    fn flavor(&self) -> Flavor {
        Flavor::Grbl
    }

    fn header(&self) -> String {
        // Absolute positioning, millimetre units, dynamic-power laser mode.
        "G90\nG21\nM4 S0\n".to_string()
    }

    fn footer(&self) -> String {
        "M5\nG0 X0 Y0\n".to_string()
    }

    fn emit(&self, op: &Op) -> String {
        match op {
            Op::RapidTo { x, y } => format!("G0 X{x:.3} Y{y:.3}"),
            Op::CutTo {
                x,
                y,
                feed_mm_s,
                power,
            } => {
                let feed_mm_min = feed_mm_s * 60.0;
                let s = self.spindle_for(*power);
                format!("G1 X{x:.3} Y{y:.3} F{feed_mm_min:.0} S{s}")
            }
            Op::LaserOff => "M5".to_string(),
        }
    }
}

/// The Marlin dialect for diode and CO2 lasers running Marlin firmware.
///
/// Marlin drives the laser with `M3`/`M5` inline spindle commands and scales power
/// to the configured `spindle_max`, which defaults to 255 on stock Marlin laser
/// builds. The move grammar matches GRBL otherwise.
#[derive(Debug, Default, Clone, Copy)]
pub struct Marlin {
    /// The S value that maps to 100 percent power. Stock Marlin laser mode is 255.
    pub spindle_max: u32,
}

impl Marlin {
    /// A Marlin dialect with the stock power range of 255.
    pub fn new() -> Self {
        Self { spindle_max: 255 }
    }

    fn spindle_for(&self, power: f64) -> u32 {
        let clamped = power.clamp(0.0, 100.0);
        (clamped / 100.0 * f64::from(self.spindle_max)).round() as u32
    }
}

impl Dialect for Marlin {
    fn flavor(&self) -> Flavor {
        Flavor::Marlin
    }

    fn header(&self) -> String {
        "G90\nG21\nM3 S0\n".to_string()
    }

    fn footer(&self) -> String {
        "M5\nG0 X0 Y0\n".to_string()
    }

    fn emit(&self, op: &Op) -> String {
        match op {
            Op::RapidTo { x, y } => format!("G0 X{x:.3} Y{y:.3}"),
            Op::CutTo {
                x,
                y,
                feed_mm_s,
                power,
            } => {
                let feed_mm_min = feed_mm_s * 60.0;
                let s = self.spindle_for(*power);
                format!("G1 X{x:.3} Y{y:.3} F{feed_mm_min:.0} S{s}")
            }
            Op::LaserOff => "M5".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grbl_cut_scales_power_and_feed() {
        let grbl = Grbl::new();
        let line = grbl.emit(&Op::CutTo {
            x: 10.0,
            y: 5.0,
            feed_mm_s: 20.0,
            power: 50.0,
        });
        assert_eq!(line, "G1 X10.000 Y5.000 F1200 S500");
    }

    #[test]
    fn program_wraps_header_and_footer() {
        let grbl = Grbl::new();
        let out = program(&grbl, &[Op::RapidTo { x: 0.0, y: 0.0 }]);
        assert!(out.starts_with("G90\nG21\nM4 S0\n"));
        assert!(out.trim_end().ends_with("G0 X0 Y0"));
    }

    #[test]
    fn marlin_scales_power_to_its_own_range() {
        let marlin = Marlin::new();
        assert_eq!(marlin.flavor(), Flavor::Marlin);
        let line = marlin.emit(&Op::CutTo {
            x: 10.0,
            y: 5.0,
            feed_mm_s: 20.0,
            power: 50.0,
        });
        // 50 percent of Marlin's 255 range is S128, feed still in mm/min.
        assert_eq!(line, "G1 X10.000 Y5.000 F1200 S128");
    }
}
