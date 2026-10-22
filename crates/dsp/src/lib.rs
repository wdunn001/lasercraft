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
