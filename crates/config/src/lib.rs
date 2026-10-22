#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! Application settings for LaserCraft.
//!
//! This is the second integration of the shared `craft-libs` workspace. The app
//! settings implement the suite's `craft_settings::Settings` trait, so LaserCraft
//! stores its configuration the same way every Craft app does: a versioned
//! `config.toml` with atomic saves, recovery, and migration handled by the shared
//! crate. The settings here round-trip through the suite's document helpers, which
//! keeps unknown keys in the file and fills missing keys from defaults.

use craft_settings::Settings;
use craft_settings::error::SectionError;
use serde::{Deserialize, Serialize};

/// The measurement units the interface presents.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Units {
    /// Millimetres.
    #[default]
    Mm,
    /// Inches.
    Inch,
}

/// LaserCraft application settings.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LaserCraftSettings {
    /// The id of the device profile to select on startup.
    pub device_profile: String,
    /// The measurement units the interface uses.
    pub units: Units,
    /// The GCode dialect to default new GCode devices to.
    pub default_flavor: String,
}

impl Settings for LaserCraftSettings {
    const APP_ID: &'static str = "lasercraft";
    const SCHEMA_VERSION: u32 = 1;
}

impl LaserCraftSettings {
    /// Render the settings as the versioned `config.toml` text.
    pub fn to_toml(&self) -> Result<String, String> {
        let mut doc = craft_settings::document::new_document(Self::SCHEMA_VERSION);
        craft_settings::document::merge_into(&mut doc, self).map_err(|e| e.to_string())?;
        Ok(craft_settings::document::render(&doc))
    }

    /// Parse settings from `config.toml` text, returning the settings and any
    /// per-section problems. A malformed document surfaces as an error.
    pub fn from_toml(text: &str) -> Result<(Self, Vec<SectionError>), String> {
        let doc = craft_settings::document::parse(text).map_err(|e| e.to_string())?;
        Ok(craft_settings::document::deserialize_sections::<Self>(&doc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_round_trip_through_toml() {
        let settings = LaserCraftSettings {
            device_profile: "ezcad2-jczfiber".to_string(),
            units: Units::Mm,
            default_flavor: "grbl".to_string(),
        };
        let toml = settings.to_toml().unwrap_or_default();
        assert!(toml.contains("ezcad2-jczfiber"));
        let parsed = LaserCraftSettings::from_toml(&toml);
        assert!(parsed.is_ok());
        if let Ok((loaded, problems)) = parsed {
            assert!(problems.is_empty());
            assert_eq!(loaded, settings);
        }
    }

    #[test]
    fn defaults_fill_missing_keys() {
        // A document with only the schema version yields the defaults.
        let parsed = LaserCraftSettings::from_toml("schema_version = 1\n");
        assert!(parsed.is_ok());
        if let Ok((loaded, _)) = parsed {
            assert_eq!(loaded, LaserCraftSettings::default());
            assert_eq!(loaded.units, Units::Mm);
        }
    }
}
