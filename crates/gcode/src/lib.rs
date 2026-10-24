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
//! Reverse engineering showed the GCode controllers share one base with a
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
    /// Mark a single dot: rapid to the point with the beam off, pulse the laser at
    /// constant power for a dwell, then turn it off. Used by dot-mode fill.
    DotMark {
        /// Target X in millimetres.
        x: f64,
        /// Target Y in millimetres.
        y: f64,
        /// Laser power as a percentage from 0.0 to 100.0.
        power: f64,
        /// Dwell time in seconds.
        dwell_s: f64,
    },
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

/// How a dialect writes laser power into its `S` word.
#[derive(Debug, Clone, Copy, PartialEq)]
enum PowerStyle {
    /// Integer `S` scaled to a maximum, for example GRBL's 1000 or Marlin's 255.
    IntScale(u32),
    /// Fractional `S` from 0.0 to 1.0, as Smoothieware expects.
    Fraction,
}

/// A configurable GCode dialect.
///
/// The GCode controllers differ only in a few details: the laser mode line in the
/// header and how power maps to the `S` word. This one type carries those as data,
/// so a new controller is a constructor, not a copied emitter. That mirrors the
/// single-base, per-dialect shape recovered from the reference binary.
#[derive(Debug, Clone, Copy)]
pub struct GCodeDialect {
    flavor: Flavor,
    laser_mode: &'static str,
    power: PowerStyle,
}

impl GCodeDialect {
    /// GRBL with dynamic-power `M4` mode and `S` scaled to 1000.
    pub fn grbl() -> Self {
        Self {
            flavor: Flavor::Grbl,
            laser_mode: "M4 S0",
            power: PowerStyle::IntScale(1000),
        }
    }

    /// GRBL constant-power `M3` mode (1.1e and earlier, diode controllers).
    pub fn grbl_m3() -> Self {
        Self {
            flavor: Flavor::Grbl,
            laser_mode: "M3 S0",
            power: PowerStyle::IntScale(1000),
        }
    }

    /// Snapmaker Artisan. The firmware is Marlin based, but it is driven as
    /// GRBL-M3 (1.1e or earlier): constant-power `M3`/`M5`. Its exported device
    /// profile sets S_Scale to 255, so `S` runs 0 to 255, not GRBL's usual 1000.
    /// Recovered from the real Snapmaker Artisan the reference tool device files.
    pub fn snapmaker() -> Self {
        Self {
            flavor: Flavor::Grbl,
            laser_mode: "M3 S0",
            power: PowerStyle::IntScale(255),
        }
    }

    /// Marlin, `M3`/`M5` with `S` scaled to the stock 255.
    pub fn marlin() -> Self {
        Self {
            flavor: Flavor::Marlin,
            laser_mode: "M3 S0",
            power: PowerStyle::IntScale(255),
        }
    }

    /// Smoothieware, `M3`/`M5` with fractional `S` from 0.0 to 1.0.
    pub fn smoothie() -> Self {
        Self {
            flavor: Flavor::Smoothie,
            laser_mode: "M3",
            power: PowerStyle::Fraction,
        }
    }

    /// GRBL-LPC, the GRBL port for LPC1768 boards. It runs GRBL firmware, so it
    /// speaks the same dynamic-power `M4` mode with `S` scaled to 1000.
    pub fn grbl_lpc() -> Self {
        Self {
            flavor: Flavor::Grbl,
            laser_mode: "M4 S0",
            power: PowerStyle::IntScale(1000),
        }
    }

    /// RepRapFirmware (Duet), `M3`/`M5` with `S` scaled to 255. Laser mode is set
    /// by `M452`, whose `R` parameter is the maximum laser value and defaults to
    /// 255, so `S` runs 0 to 255 unless the firmware config raises it. A machine
    /// whose configured maximum differs sets its own scale in the device profile.
    pub fn reprap() -> Self {
        Self {
            flavor: Flavor::RepRap,
            laser_mode: "M3 S0",
            power: PowerStyle::IntScale(255),
        }
    }

    /// LinuxCNC, `M3`/`M5` with `S` scaled to 1000, the default S-value
    /// maximum for this controller. A machine whose configured maximum differs
    /// sets its own scale in the device profile.
    pub fn linuxcnc() -> Self {
        Self {
            flavor: Flavor::LinuxCnc,
            laser_mode: "M3 S0",
            power: PowerStyle::IntScale(1000),
        }
    }

    fn power_word(&self, power: f64) -> String {
        let pct = power.clamp(0.0, 100.0);
        match self.power {
            PowerStyle::IntScale(max) => {
                format!("S{}", (pct / 100.0 * f64::from(max)).round() as u32)
            }
            PowerStyle::Fraction => format!("S{:.3}", pct / 100.0),
        }
    }
}

impl Dialect for GCodeDialect {
    fn flavor(&self) -> Flavor {
        self.flavor
    }

    fn header(&self) -> String {
        // Absolute positioning, millimetre units, then the laser mode.
        format!("G90\nG21\n{}\n", self.laser_mode)
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
                format!(
                    "G1 X{x:.3} Y{y:.3} F{feed_mm_min:.0} {}",
                    self.power_word(*power)
                )
            }
            Op::LaserOff => "M5".to_string(),
            Op::DotMark {
                x,
                y,
                power,
                dwell_s,
            } => {
                // Rapid in beam-off, fire at constant power, dwell, then off. The
                // dwell word is seconds on GRBL and LinuxCNC, milliseconds elsewhere.
                let dwell = match self.flavor {
                    Flavor::Grbl | Flavor::LinuxCnc => format!("P{dwell_s:.3}"),
                    _ => format!("P{:.0}", dwell_s * 1000.0),
                };
                format!(
                    "G0 X{x:.3} Y{y:.3}\nM3 {}\nG4 {dwell}\nM5",
                    self.power_word(*power)
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cut_50() -> Op {
        Op::CutTo {
            x: 10.0,
            y: 5.0,
            feed_mm_s: 20.0,
            power: 50.0,
        }
    }

    #[test]
    fn grbl_cut_scales_power_and_feed() {
        assert_eq!(
            GCodeDialect::grbl().emit(&cut_50()),
            "G1 X10.000 Y5.000 F1200 S500"
        );
    }

    #[test]
    fn program_wraps_header_and_footer() {
        let out = program(&GCodeDialect::grbl(), &[Op::RapidTo { x: 0.0, y: 0.0 }]);
        assert!(out.starts_with("G90\nG21\nM4 S0\n"));
        assert!(out.trim_end().ends_with("G0 X0 Y0"));
    }

    #[test]
    fn dot_mark_pulses_at_constant_power_with_a_dwell() {
        let dot = Op::DotMark {
            x: 3.0,
            y: 4.0,
            power: 60.0,
            dwell_s: 0.002,
        };
        // GRBL: rapid in beam-off, M3 at scaled power, dwell in seconds, then off.
        assert_eq!(
            GCodeDialect::grbl().emit(&dot),
            "G0 X3.000 Y4.000\nM3 S600\nG4 P0.002\nM5"
        );
        // Marlin expresses the dwell in milliseconds.
        assert!(GCodeDialect::marlin().emit(&dot).contains("G4 P2"));
    }

    #[test]
    fn grbl_m3_uses_constant_power_header() {
        let out = program(&GCodeDialect::grbl_m3(), &[Op::RapidTo { x: 0.0, y: 0.0 }]);
        assert!(out.starts_with("G90\nG21\nM3 S0\n"));
        assert!(!out.contains("M4"));
    }

    #[test]
    fn snapmaker_is_constant_power_at_s255() {
        let sm = GCodeDialect::snapmaker();
        assert_eq!(sm.flavor(), Flavor::Grbl);
        // Constant-power M3, S scaled to the device profile's 255. 50 percent is S128.
        assert_eq!(sm.emit(&cut_50()), "G1 X10.000 Y5.000 F1200 S128");
        let out = program(&GCodeDialect::snapmaker(), &[Op::RapidTo { x: 0.0, y: 0.0 }]);
        assert!(out.starts_with("G90\nG21\nM3 S0\n"));
        assert!(!out.contains("M4"));
    }

    #[test]
    fn linuxcnc_scales_power_to_1000() {
        assert_eq!(
            GCodeDialect::linuxcnc().emit(&cut_50()),
            "G1 X10.000 Y5.000 F1200 S500"
        );
        let out = program(&GCodeDialect::linuxcnc(), &[Op::RapidTo { x: 0.0, y: 0.0 }]);
        assert!(out.starts_with("G90\nG21\nM3 S0\n"));
    }

    #[test]
    fn grbl_lpc_speaks_the_same_gcode_as_grbl() {
        let ops = [Op::RapidTo { x: 0.0, y: 0.0 }];
        assert_eq!(
            program(&GCodeDialect::grbl_lpc(), &ops),
            program(&GCodeDialect::grbl(), &ops)
        );
    }

    #[test]
    fn marlin_scales_power_to_its_own_range() {
        let marlin = GCodeDialect::marlin();
        assert_eq!(marlin.flavor(), Flavor::Marlin);
        // 50 percent of Marlin's 255 range is S128.
        assert_eq!(marlin.emit(&cut_50()), "G1 X10.000 Y5.000 F1200 S128");
    }

    #[test]
    fn smoothie_uses_fractional_power() {
        let smoothie = GCodeDialect::smoothie();
        assert_eq!(smoothie.flavor(), Flavor::Smoothie);
        // Smoothieware power is a 0.0 to 1.0 fraction.
        assert_eq!(smoothie.emit(&cut_50()), "G1 X10.000 Y5.000 F1200 S0.500");
    }

    #[test]
    fn reprap_scales_power_to_the_m452_default() {
        let reprap = GCodeDialect::reprap();
        assert_eq!(reprap.flavor(), Flavor::RepRap);
        // 50 percent of the M452 default 255 range is S128.
        assert_eq!(reprap.emit(&cut_50()), "G1 X10.000 Y5.000 F1200 S128");
    }
}
