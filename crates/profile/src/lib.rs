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

/// One galvo mirror axis, from the Galvo and Basic Settings tab.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GalvoAxis {
    /// Whether this mirror drives the X axis.
    pub is_x_axis: bool,
    /// Reverse (negate) the direction.
    pub reverse: bool,
    /// Scale-matching percentage, 100.0 when true.
    pub scale_percent: f64,
    /// Barrel/pincushion bulge correction, 1.0 when none.
    pub bulge: f64,
    /// Parallelogram skew correction.
    pub skew: f64,
    /// Trapezoid correction, 1.0 when none.
    pub trapezoid: f64,
}

impl Default for GalvoAxis {
    fn default() -> Self {
        Self {
            is_x_axis: false,
            reverse: false,
            scale_percent: 100.0,
            bulge: 1.0,
            skew: 0.0,
            trapezoid: 1.0,
        }
    }
}

/// Red-dot pointer alignment.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RedDotConfig {
    /// Offset in millimetres.
    pub offset_mm: [f64; 2],
    /// Scale factors.
    pub scale: [f64; 2],
    /// Rest position in millimetres.
    pub rest_pos_mm: [f64; 2],
}

impl Default for RedDotConfig {
    fn default() -> Self {
        Self {
            offset_mm: [0.0, 0.0],
            scale: [1.0, 1.0],
            rest_pos_mm: [0.0, 0.0],
        }
    }
}

/// Jump (laser-off travel) timing defaults.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct JumpDefaults {
    /// Jump speed in millimetres per second.
    pub jump_speed_mm_s: f64,
    /// Minimum jump delay in microseconds.
    pub min_jump_delay_us: f64,
    /// Maximum jump delay in microseconds.
    pub max_jump_delay_us: f64,
    /// Distance below which the delay is not scaled, in millimetres.
    pub jump_distance_limit_mm: f64,
}

impl Default for JumpDefaults {
    fn default() -> Self {
        Self {
            jump_speed_mm_s: 4000.0,
            min_jump_delay_us: 500.0,
            max_jump_delay_us: 500.0,
            jump_distance_limit_mm: 0.0,
        }
    }
}

/// Timing-constant delays around marking.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DelayDefaults {
    /// Laser-on timing constant in microseconds.
    pub laser_on_tc_us: f64,
    /// Laser-off timing constant in microseconds.
    pub laser_off_tc_us: f64,
    /// End timing constant in microseconds.
    pub end_tc_us: f64,
    /// Polygon (corner) timing constant in microseconds.
    pub polygon_tc_us: f64,
}

impl Default for DelayDefaults {
    fn default() -> Self {
        Self {
            laser_on_tc_us: -300.0,
            laser_off_tc_us: 100.0,
            end_tc_us: 300.0,
            polygon_tc_us: 100.0,
        }
    }
}

/// One IO port assignment: a port number and its active level, or unassigned.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IoAssignment {
    /// The assigned port, 0 to 15, or none.
    pub port: Option<u8>,
    /// Whether the active level is logic high.
    pub active_high: bool,
}

/// The named galvo IO assignments, from the Ports and Laser Settings tab.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GalvoIo {
    /// Ready-light output.
    pub ready_light: IoAssignment,
    /// Busy-light output.
    pub busy_light: IoAssignment,
    /// Red-dot output.
    pub red_dot: IoAssignment,
    /// Keep the red dot on at all times.
    pub red_dot_always_on: bool,
    /// Blank the beam between marks.
    pub enable_blanking: bool,
    /// Blanking delay in microseconds.
    pub blanking_delay_us: f64,
    /// Start-framing input.
    pub start_framing: IoAssignment,
    /// Start-marking input.
    pub start_marking: IoAssignment,
    /// Require a full pulse on the start input.
    pub require_full_pulse: bool,
    /// Stop-marking input.
    pub stop_marking: IoAssignment,
    /// Done-marking output.
    pub done_marking: IoAssignment,
    /// Output pulse duration in milliseconds.
    pub duration_ms: f64,
    /// Door-protect interlock input.
    pub door_protect: IoAssignment,
    /// Second-laser-source select output.
    pub second_laser_source: IoAssignment,
}

/// Fiber laser source configuration, from the Laser Source Configuration panel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FiberLaserConfig {
    /// Fiber source vendor, for example `JPT`.
    pub fiber_type: String,
    /// Enable the MOPA Q-pulse-width setting.
    pub q_pulse_width: bool,
    /// Enable the master oscillator only while running.
    pub mo_only_when_running: bool,
    /// Master-oscillator open delay in milliseconds.
    pub open_mo_delay_ms: f64,
    /// Enable PWM frequency control.
    pub pwm_enable: bool,
    /// Maximum PWM frequency in kilohertz.
    pub max_freq_khz: f64,
    /// Minimum PWM frequency in kilohertz.
    pub min_freq_khz: f64,
    /// Enable the tickle pulse that keeps the fiber primed.
    pub tickle_enable: bool,
    /// Tickle pulse width in microseconds.
    pub tickle_pulse_us: f64,
    /// Tickle frequency in kilohertz.
    pub tickle_freq_khz: f64,
    /// Enable the first-pulse killer.
    pub fpk_enable: bool,
    /// First-pulse-killer start power percentage.
    pub fpk_start_power: f64,
    /// First-pulse-killer increment.
    pub fpk_increment: f64,
    /// UV minimum pulse in nanoseconds.
    pub uv_min_pulse_ns: f64,
    /// UV maximum pulse in nanoseconds.
    pub uv_max_pulse_ns: f64,
    /// SPI simmer current.
    pub spi_simmer_current: f64,
}

impl Default for FiberLaserConfig {
    fn default() -> Self {
        Self {
            fiber_type: String::new(),
            q_pulse_width: false,
            mo_only_when_running: false,
            open_mo_delay_ms: 8.0,
            pwm_enable: false,
            max_freq_khz: 4000.0,
            min_freq_khz: 1.0,
            tickle_enable: false,
            tickle_pulse_us: 1.0,
            tickle_freq_khz: 5.0,
            fpk_enable: false,
            fpk_start_power: 10.0,
            fpk_increment: 10.0,
            uv_min_pulse_ns: 1.0,
            uv_max_pulse_ns: 20.0,
            spi_simmer_current: 80.0,
        }
    }
}

/// Simulation and motion planning settings, from the Additional Settings tab.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SimulationConfig {
    /// Maximum speed per axis in millimetres per second.
    pub max_speed_mm_s: [f64; 2],
    /// Cut acceleration per axis in millimetres per second squared.
    pub cut_accel: [f64; 2],
    /// Engrave acceleration per axis.
    pub engrave_accel: [f64; 2],
    /// Rapid speed in millimetres per second.
    pub rapid_speed_mm_s: f64,
    /// Corner accuracy tolerance in millimetres.
    pub corner_accuracy_mm: f64,
    /// Minimum corner speed in millimetres per second.
    pub min_corner_speed_mm_s: f64,
    /// Cut speed scale percentage.
    pub cut_speed_scale_pct: f64,
    /// Global speed scale percentage.
    pub global_speed_scale_pct: f64,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            max_speed_mm_s: [7000.0, 7000.0],
            cut_accel: [1_500_000.0, 1_500_000.0],
            engrave_accel: [1_500_000.0, 1_500_000.0],
            rapid_speed_mm_s: 7000.0,
            corner_accuracy_mm: 0.010,
            min_corner_speed_mm_s: 1.0,
            cut_speed_scale_pct: 100.0,
            global_speed_scale_pct: 100.0,
        }
    }
}

/// Extension settings, from the Extensions tab.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExtensionsConfig {
    /// Enable a hardware pendant.
    pub pendant_enable: bool,
    /// Enable the permanent API access port.
    pub api_port_enable: bool,
    /// The fixed API port number, 0 for automatic.
    pub api_fixed_port: u16,
    /// Enable drag-and-drop import.
    pub dragon_drop_enable: bool,
}

/// Full galvo and fiber device configuration, matching the LightBurn galvo device
/// settings (Galvo/Basic, Ports/Laser, Additional, Extensions). Every field has a
/// sensible default, so a profile fills only what differs from stock.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GalvoConfig {
    /// Field width in millimetres (the build area X).
    pub field_width_mm: f64,
    /// Field height in millimetres.
    pub field_height_mm: f64,
    /// Field offset in millimetres.
    #[serde(default)]
    pub field_offset_mm: [f64; 2],
    /// Field rotation in degrees.
    #[serde(default)]
    pub field_angle_deg: f64,
    /// Device coordinate resolution in bits.
    pub resolution_bits: u32,
    /// Lens correction file (COR or UCF).
    #[serde(default)]
    pub correction_file: Option<String>,
    /// First galvo mirror.
    #[serde(default)]
    pub galvo1: GalvoAxis,
    /// Second galvo mirror.
    #[serde(default)]
    pub galvo2: GalvoAxis,
    /// Framing speed in millimetres per second.
    #[serde(default = "default_frame_speed")]
    pub frame_speed_mm_s: f64,
    /// Maximum speed in millimetres per second.
    #[serde(default = "default_max_speed")]
    pub max_speed_mm_s: f64,
    /// Red-dot alignment.
    #[serde(default)]
    pub red_dot: RedDotConfig,
    /// Jump timing defaults.
    #[serde(default)]
    pub jump: JumpDefaults,
    /// Marking delays.
    #[serde(default)]
    pub delays: DelayDefaults,
    /// Fiber source configuration, when the source is fiber.
    #[serde(default)]
    pub fiber: Option<FiberLaserConfig>,
    /// Named IO assignments.
    #[serde(default)]
    pub io: GalvoIo,
    /// Simulation and motion planning settings.
    #[serde(default)]
    pub simulation: SimulationConfig,
    /// Extension settings.
    #[serde(default)]
    pub extensions: ExtensionsConfig,
    /// Play a sound when a job finishes.
    #[serde(default)]
    pub play_sound_on_finish: bool,
    /// Require framing before a job starts.
    #[serde(default)]
    pub require_framing_before_start: bool,
    /// Enable the job checklist.
    #[serde(default)]
    pub job_checklist: bool,
    /// Save and load rotary settings with project files.
    #[serde(default)]
    pub save_rotary_with_project: bool,
}

fn default_frame_speed() -> f64 {
    3000.0
}

fn default_max_speed() -> f64 {
    7000.0
}

impl Default for GalvoConfig {
    fn default() -> Self {
        Self {
            field_width_mm: 110.0,
            field_height_mm: 110.0,
            field_offset_mm: [0.0, 0.0],
            field_angle_deg: 0.0,
            resolution_bits: 16,
            correction_file: None,
            galvo1: GalvoAxis::default(),
            galvo2: GalvoAxis {
                is_x_axis: true,
                ..GalvoAxis::default()
            },
            frame_speed_mm_s: default_frame_speed(),
            max_speed_mm_s: default_max_speed(),
            red_dot: RedDotConfig::default(),
            jump: JumpDefaults::default(),
            delays: DelayDefaults::default(),
            fiber: None,
            io: GalvoIo::default(),
            simulation: SimulationConfig::default(),
            extensions: ExtensionsConfig::default(),
            play_sound_on_finish: false,
            require_framing_before_start: false,
            job_checklist: false,
            save_rotary_with_project: false,
        }
    }
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

/// The camera capture backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CaptureSystem {
    /// The compatibility fallback capture path.
    Fallback,
    /// The recommended native capture path.
    #[default]
    Recommended,
}

/// Camera lens description and distortion model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CameraLens {
    /// The lens kind, for example `standard` or `fisheye`.
    pub kind: String,
    /// Radial and tangential distortion coefficients.
    #[serde(default)]
    pub distortion: Vec<f64>,
}

/// Camera-to-bed alignment from calibration markers.
///
/// The marker pixel positions and their known bed positions define the perspective
/// map from camera image to work coordinates. Pairing them per device means each
/// machine keeps its own alignment, which is the improvement over a single app-wide
/// camera.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CameraAlignment {
    /// Marker positions in camera pixels.
    pub markers_px: Vec<[f64; 2]>,
    /// The same markers in bed millimetres.
    pub markers_mm: Vec<[f64; 2]>,
}

/// A per-device camera for overlay and alignment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraConfig {
    /// The camera device name or identifier.
    pub device_name: String,
    /// The capture resolution in pixels.
    pub resolution_px: [u32; 2],
    /// The capture backend.
    #[serde(default)]
    pub capture_system: CaptureSystem,
    /// The default capture frame rate.
    #[serde(default = "default_fps")]
    pub default_fps: f64,
    /// The lens model, when calibrated.
    #[serde(default)]
    pub lens: Option<CameraLens>,
    /// The bed alignment, when calibrated.
    #[serde(default)]
    pub alignment: Option<CameraAlignment>,
}

fn default_fps() -> f64 {
    20.0
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
    /// A per-device camera for overlay and alignment, when fitted.
    #[serde(default)]
    pub camera: Option<CameraConfig>,
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
    include_str!("../profiles/commarker-b6.json"),
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
            camera: None,
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
                field_width_mm: 110.0,
                field_height_mm: 110.0,
                correction_file: Some("lens-110.cor".to_string()),
                fiber: Some(FiberLaserConfig {
                    fiber_type: "JPT".to_string(),
                    q_pulse_width: true,
                    ..FiberLaserConfig::default()
                }),
                ..GalvoConfig::default()
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
            camera: None,
        };
        let json = serde_json::to_string(&profile).unwrap_or_default();
        let back: Result<DeviceProfile, _> = serde_json::from_str(&json);
        assert!(back.is_ok());
        if let Ok(back) = back {
            assert_eq!(back, profile);
            assert_eq!(back.laser_source, Some(LaserSource::Mopa));
            assert_eq!(
                back.galvo.and_then(|g| g.fiber).map(|f| f.fiber_type),
                Some("JPT".to_string())
            );
        }
    }

    #[test]
    fn galvo_defaults_fill_when_absent_from_json() {
        // The shipped ezcad2 profile sets only field and resolution; the rest default.
        let set = ProfileSet::with_builtins();
        if let Some(galvo) = set.get("ezcad2-jczfiber").and_then(|p| p.galvo.clone()) {
            assert_eq!(galvo.field_width_mm, 110.0);
            assert_eq!(galvo.frame_speed_mm_s, 3000.0);
            assert_eq!(galvo.galvo1.scale_percent, 100.0);
            assert_eq!(galvo.delays.laser_on_tc_us, -300.0);
            assert!(galvo.fiber.is_none());
        }
    }

    #[test]
    fn per_device_camera_round_trips() {
        let json = r#"{
            "id": "fiber-cam",
            "display_name": "Fiber with camera",
            "controller": "EzCad2",
            "transport": "Usb",
            "work_area_mm": [150.0, 150.0],
            "max_speed_mm_s": 7000.0,
            "camera": {
                "device_name": "ComMarker Overlay Cam",
                "resolution_px": [1920, 1080],
                "alignment": {
                    "markers_px": [[10.0, 10.0], [1900.0, 20.0]],
                    "markers_mm": [[0.0, 0.0], [150.0, 0.0]]
                }
            }
        }"#;
        let profile: Result<DeviceProfile, _> = serde_json::from_str(json);
        assert!(profile.is_ok());
        if let Ok(profile) = profile {
            let camera = profile.camera;
            assert!(camera.is_some());
            if let Some(camera) = camera {
                assert_eq!(camera.resolution_px, [1920, 1080]);
                // Capture system and fps default when absent.
                assert_eq!(camera.capture_system, CaptureSystem::Recommended);
                assert_eq!(camera.default_fps, 20.0);
                assert_eq!(camera.alignment.map(|a| a.markers_mm.len()), Some(2));
            }
        }
    }

    #[test]
    fn commarker_b6_carries_the_fiber_and_io_config() {
        let set = ProfileSet::with_builtins();
        let galvo = set.get("commarker-b6").and_then(|p| p.galvo.clone());
        assert!(galvo.is_some());
        if let Some(galvo) = galvo {
            assert_eq!(galvo.field_width_mm, 150.0);
            assert_eq!(galvo.correction_file.as_deref(), Some("jcz15.cor"));
            assert_eq!(galvo.io.start_marking.port, Some(14));
            assert!(galvo.require_framing_before_start);
            assert_eq!(
                galvo.fiber.as_ref().map(|f| f.fiber_type.as_str()),
                Some("JPT")
            );
            assert_eq!(galvo.fiber.map(|f| f.max_freq_khz), Some(4000.0));
        }
    }
}
