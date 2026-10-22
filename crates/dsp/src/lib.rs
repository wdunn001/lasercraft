#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! DSP controller protocols for LaserCraft: Ruida, Trocen, and Topwisdom.
//!
//! Reverse engineering found these as `Protocol_Ruida`, `Protocol_Trocen`, and
//! `Protocol_Topwisdom` in `LightBurn.exe`. See `docs/rea-findings.md`. They are
//! binary controllers reached over UDP or USB. The byte-level framing is recovered
//! later from the decompiler, so this crate currently supplies the descriptors and
//! the `Protocol` wiring. Each framer lands behind the same trait when its bytes
//! are confirmed.

use lasercraft_device::{Descriptor, Protocol, TransportKind};

/// Build the descriptor for a Ruida controller.
pub fn ruida_descriptor() -> Descriptor {
    Descriptor {
        id: "ruida".to_string(),
        display_name: "Ruida DSP".to_string(),
        transport: TransportKind::Udp,
        gcode_flavor: None,
    }
}

/// Build the descriptor for a Trocen controller.
pub fn trocen_descriptor() -> Descriptor {
    Descriptor {
        id: "trocen".to_string(),
        display_name: "Trocen DSP".to_string(),
        transport: TransportKind::Usb,
        gcode_flavor: None,
    }
}

/// Build the descriptor for a Topwisdom controller.
pub fn topwisdom_descriptor() -> Descriptor {
    Descriptor {
        id: "topwisdom".to_string(),
        display_name: "Topwisdom DSP".to_string(),
        transport: TransportKind::Usb,
        gcode_flavor: None,
    }
}

/// A DSP controller protocol instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DspProtocol {
    descriptor: Descriptor,
}

impl DspProtocol {
    /// A Ruida protocol instance.
    pub fn ruida() -> Self {
        Self {
            descriptor: ruida_descriptor(),
        }
    }

    /// A Trocen protocol instance.
    pub fn trocen() -> Self {
        Self {
            descriptor: trocen_descriptor(),
        }
    }

    /// A Topwisdom protocol instance.
    pub fn topwisdom() -> Self {
        Self {
            descriptor: topwisdom_descriptor(),
        }
    }
}

impl Protocol for DspProtocol {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

/// The Ruida wire-byte scramble.
///
/// Ruida controllers lightly scramble every payload byte. The transform swaps a
/// byte's lowest and highest bit, XORs a model-specific magic value, then adds
/// one. The checksum is computed before scrambling. The magic is `0x88` for most
/// controllers (644XG, 644XS, 320, 633X, 654XG), `0x11` for the 634XG, and `0x38`
/// for the RDL9635. The algorithm is the public one documented by the Ruida
/// reverse-engineering community (EduTech wiki, jnweiger, Meerk40t); this is a
/// clean reimplementation, not a copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuidaScramble {
    /// The model magic XOR value.
    pub magic: u8,
}

impl RuidaScramble {
    /// The magic value used by most Ruida controllers.
    pub const DEFAULT_MAGIC: u8 = 0x88;

    /// A scrambler with the given magic value.
    pub fn new(magic: u8) -> Self {
        Self { magic }
    }

    /// A scrambler for a named controller model, defaulting to `0x88`.
    pub fn for_model(model: &str) -> Self {
        let magic = match model {
            "634XG" => 0x11,
            "RDL9635" => 0x38,
            _ => Self::DEFAULT_MAGIC,
        };
        Self { magic }
    }

    /// Scramble one byte for the wire.
    pub fn swizzle(&self, byte: u8) -> u8 {
        let mut b = byte;
        // Swap bit 0 and bit 7 (this three-XOR sequence is its own inverse).
        b ^= b >> 7;
        b ^= b << 7;
        b ^= b >> 7;
        b ^= self.magic;
        b.wrapping_add(1)
    }

    /// Recover one byte from the wire.
    pub fn unswizzle(&self, byte: u8) -> u8 {
        let mut b = byte.wrapping_sub(1);
        b ^= self.magic;
        b ^= b >> 7;
        b ^= b << 7;
        b ^= b >> 7;
        b
    }

    /// Scramble a buffer in place.
    pub fn swizzle_buf(&self, bytes: &mut [u8]) {
        for b in bytes.iter_mut() {
            *b = self.swizzle(*b);
        }
    }

    /// Recover a buffer in place.
    pub fn unswizzle_buf(&self, bytes: &mut [u8]) {
        for b in bytes.iter_mut() {
            *b = self.unswizzle(*b);
        }
    }
}

/// Ruida command framing.
///
/// Ruida commands are opcode bytes followed by encoded operands, then the whole
/// stream is scrambled by [`RuidaScramble`] for the wire. Absolute coordinates are
/// 35-bit micrometre values packed big-endian as five 7-bit groups; 14-bit values
/// (power, short params) pack as two 7-bit groups. The opcodes and encoding are the
/// publicly documented Ruida format (jnweiger, Meerk40t, EduTech wiki). This is a
/// clean reimplementation.
pub mod command {
    /// Pack a 35-bit micrometre coordinate as five big-endian 7-bit groups.
    pub fn encode_abs(micrometres: i64) -> [u8; 5] {
        let v = (micrometres as u64) & 0x7_ffff_ffff;
        [
            ((v >> 28) & 0x7f) as u8,
            ((v >> 21) & 0x7f) as u8,
            ((v >> 14) & 0x7f) as u8,
            ((v >> 7) & 0x7f) as u8,
            (v & 0x7f) as u8,
        ]
    }

    /// Pack a 14-bit value as two big-endian 7-bit groups.
    pub fn encode_u14(value: u16) -> [u8; 2] {
        [((value >> 7) & 0x7f) as u8, (value & 0x7f) as u8]
    }

    /// `88` move absolute to (x, y) in micrometres, laser off.
    pub fn move_abs(x_um: i64, y_um: i64) -> Vec<u8> {
        let mut out = vec![0x88];
        out.extend_from_slice(&encode_abs(x_um));
        out.extend_from_slice(&encode_abs(y_um));
        out
    }

    /// `a8` cut absolute to (x, y) in micrometres, laser on.
    pub fn cut_abs(x_um: i64, y_um: i64) -> Vec<u8> {
        let mut out = vec![0xa8];
        out.extend_from_slice(&encode_abs(x_um));
        out.extend_from_slice(&encode_abs(y_um));
        out
    }

    /// `c6 01` set laser source 1 minimum power as a percentage.
    ///
    /// Power scales onto the 14-bit range `0..=0x3fff`.
    pub fn set_power_min(percent: f64) -> Vec<u8> {
        let value = (percent.clamp(0.0, 100.0) / 100.0 * 16383.0).round() as u16;
        let mut out = vec![0xc6, 0x01];
        out.extend_from_slice(&encode_u14(value));
        out
    }
}

#[cfg(test)]
mod command_tests {
    use super::command::*;

    #[test]
    fn abs_coordinate_packs_big_endian_7bit() {
        // 1000 um = 7*128 + 104, so the low two groups are 0x07 and 0x68.
        assert_eq!(encode_abs(1000), [0x00, 0x00, 0x00, 0x07, 0x68]);
        assert_eq!(encode_abs(0), [0, 0, 0, 0, 0]);
    }

    #[test]
    fn u14_and_power_encode() {
        assert_eq!(encode_u14(0x3fff), [0x7f, 0x7f]);
        assert_eq!(encode_u14(0), [0, 0]);
        // 100 percent is the full 0x3fff.
        assert_eq!(set_power_min(100.0), vec![0xc6, 0x01, 0x7f, 0x7f]);
        // 50 percent rounds to 0x2000 -> groups 0x40, 0x00.
        assert_eq!(set_power_min(50.0), vec![0xc6, 0x01, 0x40, 0x00]);
    }

    #[test]
    fn move_and_cut_carry_their_opcodes() {
        let m = move_abs(1000, 0);
        assert_eq!(m[0], 0x88);
        assert_eq!(m.len(), 11);
        assert_eq!(&m[1..6], &[0, 0, 0, 0x07, 0x68]);
        assert_eq!(cut_abs(0, 0)[0], 0xa8);
    }
}

#[cfg(test)]
mod ruida_tests {
    use super::*;

    #[test]
    fn swizzle_matches_known_values() {
        let s = RuidaScramble::new(RuidaScramble::DEFAULT_MAGIC);
        // Hand-computed from the documented algorithm with magic 0x88.
        assert_eq!(s.swizzle(0x00), 0x89);
        assert_eq!(s.swizzle(0x01), 0x09);
    }

    #[test]
    fn swizzle_round_trips_every_byte() {
        for magic in [0x88u8, 0x11, 0x38] {
            let s = RuidaScramble::new(magic);
            for b in 0u16..=255 {
                let b = b as u8;
                assert_eq!(s.unswizzle(s.swizzle(b)), b);
            }
        }
    }

    #[test]
    fn model_magics_resolve() {
        assert_eq!(RuidaScramble::for_model("644XG").magic, 0x88);
        assert_eq!(RuidaScramble::for_model("634XG").magic, 0x11);
        assert_eq!(RuidaScramble::for_model("RDL9635").magic, 0x38);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ruida_is_udp_with_no_gcode_flavor() {
        let ruida = DspProtocol::ruida();
        assert_eq!(ruida.descriptor().id, "ruida");
        assert_eq!(ruida.descriptor().transport, TransportKind::Udp);
        assert!(ruida.descriptor().gcode_flavor.is_none());
    }
}
