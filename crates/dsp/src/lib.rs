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
