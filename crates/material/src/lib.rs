#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! Material and cut-settings library for LaserCraft.
//!
//! LightBurn keeps a material library: named cut settings keyed by material and
//! thickness so an operator can recall a known-good recipe instead of retyping
//! speed and power. This crate is that library. Settings are the document model's
//! [`CutSetting`], so a preset drops straight into a job. The library serializes to
//! JSON for sharing and backup. Preset values are machine-specific, so the library
//! ships empty by default and the operator fills it for their machine.

use lasercraft_laser_doc::CutSetting;
use serde::{Deserialize, Serialize};

/// What a preset is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Operation {
    /// Cut through the material.
    Cut,
    /// Engrave the surface.
    Engrave,
    /// Score a shallow line.
    Score,
}

/// One recalled recipe: a cut setting tagged with its material and thickness.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialPreset {
    /// The material name, for example `Plywood`.
    pub material: String,
    /// The stock thickness in millimetres.
    pub thickness_mm: f64,
    /// What the preset does.
    pub operation: Operation,
    /// The cut setting to apply.
    pub setting: CutSetting,
}

/// A library of material presets.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MaterialLibrary {
    presets: Vec<MaterialPreset>,
}

impl MaterialLibrary {
    /// An empty library.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a preset.
    pub fn add(&mut self, preset: MaterialPreset) {
        self.presets.push(preset);
    }

    /// Every preset.
    pub fn presets(&self) -> &[MaterialPreset] {
        &self.presets
    }

    /// Find a preset by material name (case-insensitive), thickness, and operation.
    ///
    /// Thickness matches within 0.05 mm so a nominal `3 mm` finds a `3.0 mm` stock.
    pub fn find(
        &self,
        material: &str,
        thickness_mm: f64,
        operation: Operation,
    ) -> Option<&MaterialPreset> {
        self.presets.iter().find(|p| {
            p.operation == operation
                && p.material.eq_ignore_ascii_case(material)
                && (p.thickness_mm - thickness_mm).abs() < 0.05
        })
    }

    /// Serialize the library to pretty JSON.
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self).map_err(|e| e.to_string())
    }

    /// Parse a library from JSON.
    pub fn from_json(text: &str) -> Result<Self, String> {
        serde_json::from_str(text).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn preset() -> MaterialPreset {
        MaterialPreset {
            material: "Plywood".to_string(),
            thickness_mm: 3.0,
            operation: Operation::Cut,
            setting: CutSetting {
                index: 0,
                name: "Plywood 3 mm Cut".to_string(),
                speed_mm_s: 12.0,
                power_min: 15.0,
                power_max: 90.0,
                num_passes: 1,
                air_assist: true,
            },
        }
    }

    #[test]
    fn find_matches_case_and_thickness_tolerance() {
        let mut library = MaterialLibrary::new();
        library.add(preset());
        // Case-insensitive material, nominal thickness within tolerance.
        assert!(library.find("plywood", 3.0, Operation::Cut).is_some());
        assert!(library.find("PLYWOOD", 3.02, Operation::Cut).is_some());
        // Wrong operation or thickness does not match.
        assert!(library.find("Plywood", 3.0, Operation::Engrave).is_none());
        assert!(library.find("Plywood", 6.0, Operation::Cut).is_none());
    }

    #[test]
    fn library_round_trips_through_json() {
        let mut library = MaterialLibrary::new();
        library.add(preset());
        let json = library.to_json().unwrap_or_default();
        let loaded = MaterialLibrary::from_json(&json);
        assert!(loaded.is_ok());
        if let Ok(loaded) = loaded {
            assert_eq!(loaded.presets().len(), 1);
            assert!(loaded.find("Plywood", 3.0, Operation::Cut).is_some());
        }
    }
}
