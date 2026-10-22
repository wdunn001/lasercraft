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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn field_edges_clamp_to_range() {
        let param = GalvoParam {
            field_size_mm: 100.0,
            resolution_bits: 16,
        };
        assert_eq!(param.logic_to_device(-50.0, 50.0), (0, 65535));
        assert_eq!(param.logic_to_device(-999.0, 999.0), (0, 65535));
    }
}
