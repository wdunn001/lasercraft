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
///
/// The variants split by the channel the bytes travel over, because that choice
/// decides which native library and which browser API can open it. A GCode
/// controller is a USB-serial CDC device, so it is [`TransportKind::Serial`] and
/// the browser reaches it with Web Serial. A galvo board is a raw-bulk USB
/// device, so it is [`TransportKind::Usb`] and the browser reaches it with
/// WebUSB. A Ruida DSP is on Ethernet, so it is [`TransportKind::Tcp`] or
/// [`TransportKind::Udp`], which no browser API can open: the web app reaches it
/// only through a local bridge. See [`TransportKind::web_api`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportKind {
    /// A USB-serial CDC port, as most GCode controllers expose. Web Serial.
    Serial,
    /// A raw-bulk USB device, as the galvo JCZ/LMC board. WebUSB.
    Usb,
    /// A Bluetooth Low Energy link. Web Bluetooth.
    Bluetooth,
    /// A TCP stream, as in `TcpTransfer`. No browser API, needs a bridge.
    Tcp,
    /// A UDP datagram channel, as in `UdpTransfer`. No browser API, needs a bridge.
    Udp,
    /// UDP with a broadcast bind, the network-laser discovery pattern.
    UdpBroadcast,
    /// A WebSocket stream, as a bridge exposes or as xTool speaks.
    WebSocket,
    /// A plain HTTP endpoint, as xTool's REST control uses.
    Http,
}

/// A browser connectivity API that can open a transport without a native helper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebApi {
    /// The Web Serial API, for USB-serial controllers.
    WebSerial,
    /// The WebUSB API, for raw-bulk USB devices such as the galvo board.
    WebUsb,
    /// The Web Bluetooth API, for BLE devices.
    WebBluetooth,
    /// A WebSocket connection, direct or to a local bridge.
    WebSocket,
    /// The Fetch API, for HTTP endpoints.
    Fetch,
}

impl TransportKind {
    /// The browser API that opens this transport directly, if one exists.
    ///
    /// TCP and UDP return `None`: a browser cannot open a raw socket, so a DSP
    /// laser on Ethernet is reached from the web only through a local bridge
    /// that relays a WebSocket onto the socket.
    pub fn web_api(self) -> Option<WebApi> {
        match self {
            TransportKind::Serial => Some(WebApi::WebSerial),
            TransportKind::Usb => Some(WebApi::WebUsb),
            TransportKind::Bluetooth => Some(WebApi::WebBluetooth),
            TransportKind::WebSocket => Some(WebApi::WebSocket),
            TransportKind::Http => Some(WebApi::Fetch),
            TransportKind::Tcp | TransportKind::Udp | TransportKind::UdpBroadcast => None,
        }
    }

    /// Whether a browser can reach this transport with no native helper.
    pub fn reachable_from_web(self) -> bool {
        self.web_api().is_some()
    }
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

    #[test]
    fn web_apis_map_to_the_right_transports() {
        assert_eq!(TransportKind::Serial.web_api(), Some(WebApi::WebSerial));
        assert_eq!(TransportKind::Usb.web_api(), Some(WebApi::WebUsb));
        assert_eq!(
            TransportKind::Bluetooth.web_api(),
            Some(WebApi::WebBluetooth)
        );
        assert_eq!(TransportKind::WebSocket.web_api(), Some(WebApi::WebSocket));
        assert_eq!(TransportKind::Http.web_api(), Some(WebApi::Fetch));
        // A raw socket has no browser API, so a DSP laser needs a local bridge.
        assert_eq!(TransportKind::Tcp.web_api(), None);
        assert_eq!(TransportKind::Udp.web_api(), None);
        assert!(!TransportKind::Tcp.reachable_from_web());
        assert!(TransportKind::Serial.reachable_from_web());
    }
}
