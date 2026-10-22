#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! xTool controller support for LaserCraft.
//!
//! Reverse engineering found `Protocol_XTool_HTTP` and `Protocol_Worker_XTool_V1`
//! and `_V2` in `LightBurn.exe`. See `docs/rea-findings.md`. Unlike the serial and
//! DSP controllers, xTool speaks HTTP with a WebSocket status bus and carries its
//! own auth, credentials, and camera commands. The transport is therefore a
//! network connection. The HTTP and WebSocket codecs land on top of this wiring.

use lasercraft_device::{Descriptor, Protocol, TransportKind};

/// The xTool protocol generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Generation {
    /// The first-generation HTTP worker.
    V1,
    /// The second-generation HTTP and WebSocket worker.
    V2,
}

/// An xTool controller protocol instance.
#[derive(Debug, Clone, PartialEq)]
pub struct XToolProtocol {
    generation: Generation,
    descriptor: Descriptor,
}

impl XToolProtocol {
    /// An xTool protocol instance for the given generation.
    pub fn new(generation: Generation) -> Self {
        let suffix = match generation {
            Generation::V1 => "v1",
            Generation::V2 => "v2",
        };
        Self {
            generation,
            descriptor: Descriptor {
                id: format!("xtool-{suffix}"),
                display_name: "xTool".to_string(),
                transport: TransportKind::Tcp,
                gcode_flavor: None,
            },
        }
    }

    /// The protocol generation.
    pub fn generation(&self) -> Generation {
        self.generation
    }
}

impl Protocol for XToolProtocol {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generations_get_distinct_ids_over_tcp() {
        let v1 = XToolProtocol::new(Generation::V1);
        let v2 = XToolProtocol::new(Generation::V2);
        assert_eq!(v1.descriptor().id, "xtool-v1");
        assert_eq!(v2.descriptor().id, "xtool-v2");
        assert_eq!(v2.descriptor().transport, TransportKind::Tcp);
    }
}
