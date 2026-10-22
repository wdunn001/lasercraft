#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! Device abstraction for LaserCraft.
//!
//! This crate is the faithful translation of LightBurn's controller layer, which
//! reverse engineering recovered as a `Protocol` base class with a
//! `ProtocolDescriptor` registry, one descriptor per supported device. See
//! `docs/rea-findings.md`. The transport stack mirrors `transfer.dll`, which
//! carries TCP, UDP with broadcast discovery, and USB.
//!
//! A controller protocol lives in its own crate (`lasercraft-gcode`,
//! `lasercraft-dsp`, `lasercraft-galvo`, `lasercraft-xtool`) and implements
//! [`Protocol`]. This crate holds only the shared contracts and opens no sockets
//! of its own, following the craft-libs house style where the wire format is
//! separated from the socket.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Errors surfaced by a transport or a protocol.
#[derive(Debug, thiserror::Error)]
pub enum DeviceError {
    /// The transport could not be opened or reached.
    #[error("transport: {0}")]
    Transport(String),
    /// The job could not be framed for this controller.
    #[error("framing: {0}")]
    Framing(String),
    /// The device reported an error during a job.
    #[error("device: {0}")]
    Device(String),
}

/// How a controller is reached physically.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportKind {
    /// USB serial or raw USB, as in `transfer.dll` USB descriptor handling.
    Usb,
    /// A TCP stream, as in `TcpTransfer`.
    Tcp,
    /// A UDP datagram channel, as in `UdpTransfer`.
    Udp,
    /// UDP with a broadcast bind, the network-laser discovery pattern.
    UdpBroadcast,
}

/// A byte channel to a controller. Implementors own the socket or handle.
///
/// Implementations live outside this crate so the contracts stay free of IO,
/// which keeps the crate usable from the WebAssembly build.
pub trait Transport {
    /// Open the channel to the device.
    fn open(&mut self) -> Result<(), DeviceError>;
    /// Write a frame to the device.
    fn send(&mut self, bytes: &[u8]) -> Result<(), DeviceError>;
    /// Read available bytes into `buf`, returning the count read.
    fn recv(&mut self, buf: &mut [u8]) -> Result<usize, DeviceError>;
    /// Close the channel.
    fn close(&mut self) -> Result<(), DeviceError>;
}

/// The static description of a supported device.
///
/// This is the data-driven analogue of LightBurn's `ProtocolDescriptor` and the
/// embedded device-profile JSON. The registry is data, never hardcoded branches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Descriptor {
    /// Stable identifier, for example `grbl` or `ruida`.
    pub id: String,
    /// Human-readable name shown in the UI, for example `Emblaser 2 / Core`.
    pub display_name: String,
    /// The transport the device uses.
    pub transport: TransportKind,
    /// The GCode flavor when the device is a GCode controller, for example `grbl`.
    pub gcode_flavor: Option<String>,
}

/// A controller protocol. One implementation per controller family.
///
/// The job and layer types come from `lasercraft-laser-doc` and are threaded in
/// by the application, so this crate does not depend on the document model.
pub trait Protocol {
    /// The device this protocol instance drives.
    fn descriptor(&self) -> &Descriptor;
}

/// The set of known devices, keyed by [`Descriptor::id`].
///
/// Populated from descriptor data at startup, matching the house rule against
/// hardcoded device registries.
#[derive(Debug, Default)]
pub struct DescriptorRegistry {
    entries: BTreeMap<String, Descriptor>,
}

impl DescriptorRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace a descriptor, returning the previous one if present.
    pub fn insert(&mut self, descriptor: Descriptor) -> Option<Descriptor> {
        self.entries.insert(descriptor.id.clone(), descriptor)
    }

    /// Look up a descriptor by id.
    pub fn get(&self, id: &str) -> Option<&Descriptor> {
        self.entries.get(id)
    }

    /// Iterate every known descriptor.
    pub fn iter(&self) -> impl Iterator<Item = &Descriptor> {
        self.entries.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_round_trips_a_descriptor() {
        let mut registry = DescriptorRegistry::new();
        let grbl = Descriptor {
            id: "grbl".to_string(),
            display_name: "GRBL".to_string(),
            transport: TransportKind::Usb,
            gcode_flavor: Some("grbl".to_string()),
        };
        assert!(registry.insert(grbl.clone()).is_none());
        assert_eq!(registry.get("grbl"), Some(&grbl));
        assert_eq!(registry.iter().count(), 1);
    }
}
