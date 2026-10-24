# Connecting to machines from the web

LaserCraft runs in the browser. A browser cannot open a serial port or a USB
device the way a native process can. It offers its own device APIs instead. This
note records how LaserCraft reaches a controller from the web. It also records
which API carries each kind of machine.

## The shape of the solution

The design follows the pattern SolidCraft proved in `apps/solidcraft-web`. The
core crates stay free of input and output. They encode a job into controller
bytes and nothing more. The web app owns every device call. It reaches the
browser APIs from Rust through `web-sys`. No hand-written JavaScript is involved.

The `lasercraft-device` crate holds the `Transport` trait and the protocol
encoders. It never calls a socket. It therefore compiles to
`wasm32-unknown-unknown` untouched. A native build supplies a `Transport` backed
by a serial or USB library. A web build supplies a `Transport` backed by a
browser API. The core above the transport is the same in both.

The browser device APIs are asynchronous. The encoders are synchronous. The core
produces a byte frame and hands it to the web app. The web app then performs the
asynchronous write. The file layer already works this way. The Rust core reads a
file from memory. The host fills that memory from the browser's storage or a file
picker first.

## Which API reaches which machine

`TransportKind::web_api` maps each transport to the browser API that opens it.
The connect screen reads that map to offer the right button for the machine.

| Machine | Transport | Browser API |
|:--|:--|:--|
| GCode controller (GRBL, Marlin, Smoothie) | Serial | Web Serial |
| Galvo board (EZCAD2 / JCZFiber, LMC) | Usb | WebUSB |
| BLE hobby laser | Bluetooth | Web Bluetooth |
| xTool | WebSocket | WebSocket |
| xTool REST | Http | Fetch |
| Ruida or Trocen or TopWisdom DSP | Tcp or Udp | none, see below |

The galvo case is the interesting one. The JCZ/LMC board is a raw-bulk USB
device. WebUSB can claim its interface from the page when no kernel driver holds
it. The browser then drives the fiber laser directly.

## The DSP gap

A Ruida DSP sits on Ethernet over TCP and UDP. No browser API opens a raw socket.
`web_api` returns `None` for those transports. The web app reaches a DSP laser
through a small local bridge. The bridge is a native helper on the operator's
machine. It accepts a WebSocket from the page. It relays the bytes onto the TCP
or UDP socket. The page speaks `TransportKind::WebSocket` to the bridge. The
bridge speaks the DSP's own transport to the laser.

## Enabling the APIs

Web Serial, WebUSB and Web Bluetooth sit behind the `web-sys` unstable flag. A
web build sets `RUSTFLAGS=--cfg=web_sys_unstable_apis`. It enables the `web-sys`
features for those three APIs. The Gamepad and File System Access APIs that
SolidCraft already uses are stable. They need no flag. Each API also needs a user
gesture to grant access. A connect button triggers the device picker.
