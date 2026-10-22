#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! Configurable device profiles for LaserCraft.
//!
//! Like the other applications in this space, a device is a data-driven profile
//! the user can edit and add to, not a hardcoded branch. A profile names a
//! controller, how it is reached, its work area, and its limits. Galvo and fiber
//! profiles carry a [`GalvoConfig`]. LaserCraft ships built-in profiles and loads
//! user profiles from a directory, so adding a machine is dropping in a JSON file.
//!
//! The controller set matches the devices recovered from LightBurn. See
//! `docs/rea-findings.md`.

use lasercraft_device::{Descriptor, TransportKind};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

/// The controller a device speaks, covering the full supported set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Controller {
    /// GRBL and compatible.
    Grbl,
    /// GRBL on an LPC board.
    GrblLpc,
    /// GRBL M3 (firmware 1.1e or earlier).
    GrblM3,
    /// Marlin firmware.
    Marlin,
    /// Snapmaker's Marlin variant.
    SnapMaker,
    /// Smoothieware.
    Smoothie,
    /// RepRapFirmware.
    RepRap,
    /// LinuxCNC.
    LinuxCnc,
    /// Ruida DSP.
    Ruida,
    /// Trocen AWC DSP.
    Trocen,
    /// Topwisdom DSP.
    Topwisdom,
    /// EZCAD2 fiber galvo on a JCZ board.
    EzCad2,
    /// JCZ fiber galvo.
    JczFiber,
    /// BSL fiber galvo.
    BslFiber,
    /// Generic galvo.
    Galvo,
    /// xTool HTTP devices.
    XTool,
    /// Epilog.
    Epilog,
    /// ACCELaser.
    AcceLaser,
    /// CanCam.
    CanCam,
    /// Cubiio.
    Cubiio,
    /// FabKit.
    FabKit,
    /// Gerbil STM.
    GerbilStm,
    /// iLaser.
    ILaser,
    /// LBP.
    Lbp,
    /// ZhiYuan.
    ZhiYuan,
}

impl Controller {
    /// Whether the controller is a galvo or fiber marker.
    pub fn is_galvo(self) -> bool {
        matches!(
            self,
            Controller::EzCad2 | Controller::JczFiber | Controller::BslFiber | Controller::Galvo
        )
    }

    /// Whether the controller is driven by a GCode dialect.
    pub fn is_gcode(self) -> bool {
        matches!(
            self,
            Controller::Grbl
                | Controller::GrblLpc
                | Controller::GrblM3
                | Controller::Marlin
                | Controller::SnapMaker
                | Controller::Smoothie
                | Controller::RepRap
                | Controller::LinuxCnc
                | Controller::Cubiio
                | Controller::FabKit
                | Controller::AcceLaser
                | Controller::CanCam
                | Controller::ZhiYuan
                | Controller::GerbilStm
                | Controller::ILaser
        )
    }
}

/// The laser source a machine carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LaserSource {
    /// A CO2 tube.
    Co2,
    /// A diode module.
    Diode,
    /// A Q-switched fiber source.
    Fiber,
    /// A MOPA fiber source with adjustable pulse width.
    Mopa,
    /// An ultraviolet source.
    Uv,
}

/// Galvo field configuration for a fiber or galvo device.
///
/// The field is the galvo build area. Scale matching corrects the X and Y gains
/// so a commanded length marks true, the angle offset squares a rotated head, and
/// a correction (COR or UCF) file supplies the lens distortion map.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GalvoConfig {
    /// The square field side length in millimetres (the build area).
    pub field_size_mm: f64,
    /// The device coordinate resolution in bits.
    pub resolution_bits: u32,
    /// X-axis scale-matching factor, 1.0 when no correction is needed.
    #[serde(default = "one")]
    pub scale_x: f64,
    /// Y-axis scale-matching factor.
    #[serde(default = "one")]
    pub scale_y: f64,
    /// Field rotation offset in degrees to square the head.
    #[serde(default)]
    pub angle_offset_deg: f64,
    /// Path to the lens correction file (COR or UCF), if any.
    #[serde(default)]
    pub correction_file: Option<String>,
}

fn one() -> f64 {
    1.0
}

/// A rotary attachment, for engraving cylinders.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RotaryConfig {
    /// Which machine axis the rotary replaces, for example `A` or `Y`.
    pub axis: char,
    /// Travel in millimetres for one full rotation of the roller or chuck.
    pub mm_per_rotation: f64,
    /// The object diameter in millimetres, used to map surface distance to rotation.
    pub object_diameter_mm: f64,
}

/// A configurable input or output port.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IoPort {
    /// A human label, for example `Air Assist` or `Door Interlock`.
    pub name: String,
    /// The hardware pin or port number.
    pub pin: u8,
    /// Whether the active level is logic high.
    pub active_high: bool,
}

/// A device profile: everything needed to drive one machine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceProfile {
    /// Stable identifier.
    pub id: String,
    /// Human-readable name.
    pub display_name: String,
    /// The controller the device speaks.
    pub controller: Controller,
    /// How the device is reached.
    pub transport: TransportKind,
    /// The work area as `[width, height]` in millimetres.
    pub work_area_mm: [f64; 2],
    /// The machine maximum speed in millimetres per second.
    pub max_speed_mm_s: f64,
    /// The GCode flavor for GCode controllers.
    #[serde(default)]
    pub gcode_flavor: Option<String>,
    /// Galvo field configuration for galvo and fiber controllers.
    #[serde(default)]
    pub galvo: Option<GalvoConfig>,
    /// The laser source, when known.
    #[serde(default)]
    pub laser_source: Option<LaserSource>,
    /// A rotary attachment, when fitted.
    #[serde(default)]
    pub rotary: Option<RotaryConfig>,
    /// Extra motion axes beyond X and Y, for example `["Z", "U"]`.
    #[serde(default)]
    pub extra_axes: Vec<String>,
    /// Configurable input and output ports.
    #[serde(default)]
    pub io_ports: Vec<IoPort>,
}

impl DeviceProfile {
    /// The device descriptor for the registry and protocol layer.
    pub fn descriptor(&self) -> Descriptor {
        Descriptor {
            id: self.id.clone(),
            display_name: self.display_name.clone(),
            transport: self.transport.clone(),
            gcode_flavor: self.gcode_flavor.clone(),
        }
    }
}

const BUILTIN: &[&str] = &[
    include_str!("../profiles/snapmaker.json"),
    include_str!("../profiles/ezcad2-jczfiber.json"),
    include_str!("../profiles/grbl.json"),
    include_str!("../profiles/marlin.json"),
    include_str!("../profiles/ruida.json"),
];

/// The profiles shipped with LaserCraft. A profile that fails to parse is skipped.
pub fn builtin_profiles() -> Vec<DeviceProfile> {
    BUILTIN
        .iter()
        .filter_map(|json| serde_json::from_str(json).ok())
        .collect()
}

/// Load every `*.json` device profile in a directory, skipping unreadable or
/// invalid files so one bad profile does not block the rest.
pub fn load_dir(dir: &Path) -> Vec<DeviceProfile> {
    let mut profiles = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return profiles;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        if let Ok(profile) = serde_json::from_str::<DeviceProfile>(&text) {
            profiles.push(profile);
        }
    }
    profiles
}

/// A set of device profiles keyed by id, built-ins plus user additions.
#[derive(Debug, Clone, Default)]
pub struct ProfileSet {
    profiles: BTreeMap<String, DeviceProfile>,
}

impl ProfileSet {
    /// A set seeded with the built-in profiles.
    pub fn with_builtins() -> Self {
        let mut set = Self::default();
        for profile in builtin_profiles() {
            set.insert(profile);
        }
        set
    }

    /// Insert or replace a profile, returning the previous one if present.
    pub fn insert(&mut self, profile: DeviceProfile) -> Option<DeviceProfile> {
        self.profiles.insert(profile.id.clone(), profile)
    }

    /// Merge every profile from a user directory, overriding built-ins by id.
    pub fn load_user_dir(&mut self, dir: &Path) {
        for profile in load_dir(dir) {
            self.insert(profile);
        }
    }

    /// Look up a profile by id.
    pub fn get(&self, id: &str) -> Option<&DeviceProfile> {
        self.profiles.get(id)
    }

    /// Iterate every profile.
    pub fn iter(&self) -> impl Iterator<Item = &DeviceProfile> {
        self.profiles.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ships_the_priority_profiles() {
        let set = ProfileSet::with_builtins();
        let snapmaker = set.get("snapmaker");
        assert!(snapmaker.is_some());
        assert_eq!(snapmaker.map(|p| p.controller), Some(Controller::SnapMaker));

        let fiber = set.get("ezcad2-jczfiber");
        assert!(fiber.is_some());
        if let Some(fiber) = fiber {
            assert_eq!(fiber.controller, Controller::EzCad2);
            assert!(fiber.controller.is_galvo());
            assert!(fiber.galvo.is_some());
        }
    }

    #[test]
    fn snapmaker_uses_the_marlin_flavor() {
        let set = ProfileSet::with_builtins();
        let flavor = set.get("snapmaker").and_then(|p| p.gcode_flavor.clone());
        assert_eq!(flavor.as_deref(), Some("marlin"));
    }

    #[test]
    fn a_user_profile_overrides_a_builtin_by_id() {
        let mut set = ProfileSet::with_builtins();
        let custom = DeviceProfile {
            id: "grbl".to_string(),
            display_name: "My Tuned GRBL".to_string(),
            controller: Controller::Grbl,
            transport: TransportKind::Usb,
            work_area_mm: [500.0, 500.0],
            max_speed_mm_s: 250.0,
            gcode_flavor: Some("grbl".to_string()),
            galvo: None,
            laser_source: Some(LaserSource::Diode),
            rotary: None,
            extra_axes: Vec::new(),
            io_ports: Vec::new(),
        };
        set.insert(custom);
        assert_eq!(
            set.get("grbl").map(|p| p.display_name.as_str()),
            Some("My Tuned GRBL")
        );
    }

    #[test]
    fn fiber_config_round_trips_with_galvo_rotary_and_io() {
        let profile = DeviceProfile {
            id: "fiber-20w".to_string(),
            display_name: "Fiber 20W".to_string(),
            controller: Controller::JczFiber,
            transport: TransportKind::Usb,
            work_area_mm: [110.0, 110.0],
            max_speed_mm_s: 7000.0,
            gcode_flavor: None,
            galvo: Some(GalvoConfig {
                field_size_mm: 110.0,
                resolution_bits: 16,
                scale_x: 1.002,
                scale_y: 0.998,
                angle_offset_deg: 0.25,
                correction_file: Some("lens-110.cor".to_string()),
            }),
            laser_source: Some(LaserSource::Mopa),
            rotary: Some(RotaryConfig {
                axis: 'A',
                mm_per_rotation: 100.0,
                object_diameter_mm: 31.8,
            }),
            extra_axes: vec!["Z".to_string()],
            io_ports: vec![IoPort {
                name: "Air Assist".to_string(),
                pin: 2,
                active_high: true,
            }],
        };
        let json = serde_json::to_string(&profile).unwrap_or_default();
        let back: Result<DeviceProfile, _> = serde_json::from_str(&json);
        assert!(back.is_ok());
        if let Ok(back) = back {
            assert_eq!(back, profile);
            assert_eq!(back.laser_source, Some(LaserSource::Mopa));
            assert_eq!(back.galvo.map(|g| g.scale_x), Some(1.002));
        }
    }

    #[test]
    fn galvo_defaults_fill_when_absent_from_json() {
        // The shipped ezcad2 profile omits the new galvo fields; they default.
        let set = ProfileSet::with_builtins();
        if let Some(galvo) = set.get("ezcad2-jczfiber").and_then(|p| p.galvo.clone()) {
            assert_eq!(galvo.scale_x, 1.0);
            assert_eq!(galvo.scale_y, 1.0);
            assert_eq!(galvo.angle_offset_deg, 0.0);
            assert!(galvo.correction_file.is_none());
        }
    }
}
