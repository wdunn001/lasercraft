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
//! LightBurn's material library is organised per machine, then by material, then by
//! the result an operator recorded, since the same material cuts differently on a
//! different machine. This library keeps that hierarchy: a machine id maps to
//! material entries, each entry naming the material, thickness, operation, and the
//! recorded result, with the [`CutSetting`] that achieved it. Entries serialize to
//! JSON for sharing. Values are machine-specific, so the library ships empty and the
//! operator fills it from material tests.

use lasercraft_laser_doc::CutSetting;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// What an entry is for.
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

/// One recorded result: the settings that achieved a named outcome on a material.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialEntry {
    /// The material name, for example `Plywood`.
    pub material: String,
    /// The stock thickness in millimetres.
    pub thickness_mm: f64,
    /// What the entry does.
    pub operation: Operation,
    /// The recorded result, for example `Clean through-cut` or `Dark engrave`.
    pub result: String,
    /// The cut setting that produced the result.
    pub setting: CutSetting,
    /// Free-form operator notes.
    #[serde(default)]
    pub notes: String,
}

/// A material library organised by machine, then material, then result.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MaterialLibrary {
    by_machine: BTreeMap<String, Vec<MaterialEntry>>,
}

impl MaterialLibrary {
    /// An empty library.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an entry under a machine.
    pub fn add(&mut self, machine_id: impl Into<String>, entry: MaterialEntry) {
        self.by_machine
            .entry(machine_id.into())
            .or_default()
            .push(entry);
    }

    /// The machine ids that have entries.
    pub fn machines(&self) -> impl Iterator<Item = &str> {
        self.by_machine.keys().map(String::as_str)
    }

    /// The distinct material names recorded for a machine, sorted.
    pub fn materials(&self, machine_id: &str) -> Vec<&str> {
        let mut names: Vec<&str> = self
            .by_machine
            .get(machine_id)
            .into_iter()
            .flatten()
            .map(|e| e.material.as_str())
            .collect();
        names.sort_unstable();
        names.dedup();
        names
    }

    /// Every recorded result for a material on a machine.
    pub fn results(&self, machine_id: &str, material: &str) -> Vec<&MaterialEntry> {
        self.by_machine
            .get(machine_id)
            .into_iter()
            .flatten()
            .filter(|e| e.material.eq_ignore_ascii_case(material))
            .collect()
    }

    /// Find one entry by machine, material, thickness (within 0.05 mm), and operation.
    pub fn find(
        &self,
        machine_id: &str,
        material: &str,
        thickness_mm: f64,
        operation: Operation,
    ) -> Option<&MaterialEntry> {
        self.by_machine
            .get(machine_id)
            .into_iter()
            .flatten()
            .find(|e| {
                e.operation == operation
                    && e.material.eq_ignore_ascii_case(material)
                    && (e.thickness_mm - thickness_mm).abs() < 0.05
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

    fn entry(material: &str, thickness: f64, result: &str) -> MaterialEntry {
        MaterialEntry {
            material: material.to_string(),
            thickness_mm: thickness,
            operation: Operation::Cut,
            result: result.to_string(),
            setting: CutSetting {
                index: 0,
                name: format!("{material} {thickness} Cut"),
                speed_mm_s: 12.0,
                power_min: 15.0,
                power_max: 90.0,
                num_passes: 1,
                air_assist: true,
                galvo: None,
            },
            notes: String::new(),
        }
    }

    fn library() -> MaterialLibrary {
        let mut lib = MaterialLibrary::new();
        lib.add("snapmaker", entry("Plywood", 3.0, "Clean cut"));
        lib.add("snapmaker", entry("Acrylic", 3.0, "Flame-polished edge"));
        lib.add("ezcad2-jczfiber", entry("Steel", 0.0, "Dark anneal"));
        lib
    }

    #[test]
    fn hierarchy_is_machine_then_material_then_result() {
        let lib = library();
        let machines: Vec<&str> = lib.machines().collect();
        assert_eq!(machines, vec!["ezcad2-jczfiber", "snapmaker"]);
        // Materials are scoped to the machine.
        assert_eq!(lib.materials("snapmaker"), vec!["Acrylic", "Plywood"]);
        assert_eq!(lib.materials("ezcad2-jczfiber"), vec!["Steel"]);
        // Results drill into one material on one machine.
        let plywood = lib.results("snapmaker", "plywood");
        assert_eq!(plywood.len(), 1);
        assert_eq!(
            plywood.first().map(|e| e.result.as_str()),
            Some("Clean cut")
        );
    }

    #[test]
    fn find_is_scoped_to_the_machine() {
        let lib = library();
        assert!(
            lib.find("snapmaker", "Plywood", 3.0, Operation::Cut)
                .is_some()
        );
        // The same material on a machine that lacks it does not match.
        assert!(
            lib.find("ezcad2-jczfiber", "Plywood", 3.0, Operation::Cut)
                .is_none()
        );
    }

    #[test]
    fn library_round_trips_through_json() {
        let json = library().to_json().unwrap_or_default();
        let loaded = MaterialLibrary::from_json(&json);
        assert!(loaded.is_ok());
        if let Ok(loaded) = loaded {
            assert_eq!(loaded.materials("snapmaker"), vec!["Acrylic", "Plywood"]);
        }
    }
}
