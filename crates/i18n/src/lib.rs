#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! Localization for LaserCraft.
//!
//! The Craft apps localize through a translation table keyed by a string id, as
//! noted for the suite in `docs/rea-findings.md` and the house `tr` convention.
//! This crate holds a [`Catalog`] of per-locale tables with a deterministic
//! fallback: the requested locale, then the default locale, then the key itself.
//! Returning the key keeps a missing translation visible and never panics.

use serde::Deserialize;
use std::collections::BTreeMap;

/// A BCP-47 locale tag, for example `en` or `pt-BR`.
pub type Locale = String;

/// A set of translation tables with a default locale for fallback.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Catalog {
    default_locale: Locale,
    tables: BTreeMap<Locale, BTreeMap<String, String>>,
}

impl Catalog {
    /// Create a catalog whose fallback locale is `default_locale`.
    pub fn new(default_locale: impl Into<Locale>) -> Self {
        Self {
            default_locale: default_locale.into(),
            tables: BTreeMap::new(),
        }
    }

    /// Install or replace the table for a locale.
    pub fn add_table(&mut self, locale: impl Into<Locale>, table: BTreeMap<String, String>) {
        self.tables.insert(locale.into(), table);
    }

    /// The configured fallback locale.
    pub fn default_locale(&self) -> &str {
        &self.default_locale
    }

    /// Translate `key` for `locale`.
    ///
    /// Resolution order is the requested locale, then the default locale, then the
    /// key itself. The key is returned borrowed so a miss is never silently blank.
    pub fn tr<'a>(&'a self, locale: &str, key: &'a str) -> &'a str {
        if let Some(value) = self.lookup(locale, key) {
            return value;
        }
        if let Some(value) = self.lookup(&self.default_locale, key) {
            return value;
        }
        key
    }

    fn lookup(&self, locale: &str, key: &str) -> Option<&str> {
        self.tables
            .get(locale)
            .and_then(|table| table.get(key))
            .map(String::as_str)
    }
}

/// The locales shipped with LaserCraft, embedded at compile time.
const BUILTIN_LOCALES: &[(&str, &str)] = &[
    ("en", include_str!("../locales/en.json")),
    ("es", include_str!("../locales/es.json")),
    ("de", include_str!("../locales/de.json")),
    ("fr", include_str!("../locales/fr.json")),
    ("pt-BR", include_str!("../locales/pt-BR.json")),
];

/// Build the catalog shipped with LaserCraft, with English as the fallback.
///
/// The locale tables are embedded JSON. A table that fails to parse is skipped so
/// a packaging mistake degrades to the fallback rather than crashing.
pub fn builtin_catalog() -> Catalog {
    let mut catalog = Catalog::new("en");
    for (locale, json) in BUILTIN_LOCALES {
        if let Ok(table) = serde_json::from_str::<BTreeMap<String, String>>(json) {
            catalog.add_table(*locale, table);
        }
    }
    catalog
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalog() -> Catalog {
        let mut catalog = Catalog::new("en");
        let mut en = BTreeMap::new();
        en.insert("menu.file".to_string(), "File".to_string());
        en.insert("menu.cut".to_string(), "Cut".to_string());
        catalog.add_table("en", en);
        let mut es = BTreeMap::new();
        es.insert("menu.file".to_string(), "Archivo".to_string());
        catalog.add_table("es", es);
        catalog
    }

    #[test]
    fn resolves_requested_locale() {
        assert_eq!(catalog().tr("es", "menu.file"), "Archivo");
    }

    #[test]
    fn falls_back_to_default_then_key() {
        let catalog = catalog();
        // Missing in es, present in the en default.
        assert_eq!(catalog.tr("es", "menu.cut"), "Cut");
        // Missing everywhere returns the key.
        assert_eq!(catalog.tr("fr", "menu.unknown"), "menu.unknown");
    }

    #[test]
    fn builtin_catalog_has_real_translations() {
        let catalog = builtin_catalog();
        assert_eq!(catalog.tr("en", "app.name"), "LaserCraft");
        assert_eq!(catalog.tr("de", "menu.file"), "Datei");
        assert_eq!(catalog.tr("es", "mode.engrave"), "Grabar");
        assert_eq!(catalog.tr("fr", "action.save"), "Enregistrer");
        assert_eq!(catalog.tr("pt-BR", "layer.power"), "Potência");
        // An untranslated key falls back to the English default.
        assert_eq!(catalog.tr("de", "mode.scan"), "Scannen");
    }
}
