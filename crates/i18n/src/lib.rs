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
    /// Resolution order is the requested locale, then its language prefix (so
    /// `de-DE` finds the `de` table), then the default locale, then the key
    /// itself. The key is returned borrowed so a miss is never silently blank.
    pub fn tr<'a>(&'a self, locale: &str, key: &'a str) -> &'a str {
        if let Some(value) = self.lookup(locale, key) {
            return value;
        }
        if let Some((language, _region)) = locale.split_once('-')
            && let Some(value) = self.lookup(language, key)
        {
            return value;
        }
        if let Some(value) = self.lookup(&self.default_locale, key) {
            return value;
        }
        key
    }

    /// Translate `key` for `locale`, then substitute `{name}` placeholders from
    /// `args` with their values. A placeholder with no matching arg is left in
    /// place so a wiring mistake stays visible. Resolution matches [`Catalog::tr`].
    pub fn tr_args(&self, locale: &str, key: &str, args: &[(&str, &str)]) -> String {
        let mut text = self.tr(locale, key).to_string();
        for (name, value) in args {
            text = text.replace(&format!("{{{name}}}"), value);
        }
        text
    }

    fn lookup(&self, locale: &str, key: &str) -> Option<&str> {
        self.tables
            .get(locale)
            .and_then(|table| table.get(key))
            .map(String::as_str)
    }
}

/// Normalise a system locale string such as `de_DE.UTF-8` or `pt_BR` to a
/// catalog tag such as `de-DE` or `pt-BR`. The part before a `.` or `@` is
/// kept, `_` becomes `-`, the language is lowercased and the region, when
/// present, is uppercased. [`Catalog::tr`] then resolves the region form down
/// to the language table when only the language is translated.
pub fn normalize_locale(raw: &str) -> Locale {
    let base = raw
        .split(['.', '@'])
        .next()
        .unwrap_or(raw)
        .replace('_', "-");
    let mut parts = base.split('-');
    let language = parts.next().unwrap_or("").to_ascii_lowercase();
    match parts.next() {
        Some(region) if !region.is_empty() => format!("{language}-{}", region.to_ascii_uppercase()),
        _ => language,
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

    #[test]
    fn interpolates_named_placeholders() {
        let catalog = builtin_catalog();
        let msg = catalog.tr_args(
            "en",
            "cli.error.load",
            &[("path", "job.lbrn"), ("error", "no such file")],
        );
        assert_eq!(msg, "could not load job.lbrn: no such file");
    }

    #[test]
    fn interpolates_in_a_translated_locale() {
        let catalog = builtin_catalog();
        let msg = catalog.tr_args(
            "es",
            "cli.estimate.time",
            &[("minutes", "3"), ("seconds", "12.0")],
        );
        assert_eq!(msg, "Tiempo de corte estimado: 3m 12.0s");
    }

    #[test]
    fn missing_placeholder_arg_is_left_visible() {
        let catalog = builtin_catalog();
        let msg = catalog.tr_args("en", "cli.error.load", &[("path", "a.lbrn")]);
        assert_eq!(msg, "could not load a.lbrn: {error}");
    }

    #[test]
    fn region_locale_falls_back_to_language() {
        let catalog = builtin_catalog();
        // There is no de-DE table, but the de table carries the key.
        assert_eq!(catalog.tr("de-DE", "menu.file"), "Datei");
    }

    #[test]
    fn normalizes_system_locale_strings() {
        assert_eq!(normalize_locale("de_DE.UTF-8"), "de-DE");
        assert_eq!(normalize_locale("pt_BR"), "pt-BR");
        assert_eq!(normalize_locale("fr"), "fr");
        assert_eq!(normalize_locale("en_US@posix"), "en-US");
    }
}
