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

/// Galvo field configuration for a fiber or galvo device.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GalvoConfig {
    /// The square field side length in millimetres.
    pub field_size_mm: f64,
    /// The device coordinate resolution in bits.
    pub resolution_bits: u32,
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
        };
        set.insert(custom);
        assert_eq!(
            set.get("grbl").map(|p| p.display_name.as_str()),
            Some("My Tuned GRBL")
        );
    }
}
