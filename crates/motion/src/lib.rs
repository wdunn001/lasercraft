#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! Motion planning for LaserCraft.
//!
//! Reverse engineering of `acclplanner.dll` showed the planner is configured by
//! an acceleration and a maximum speed, and it emits Klipper limits such as
//! `SET_VELOCITY_LIMIT ACCEL=` and `ACCEL_TO_DECEL=`. See `docs/rea-findings.md`.
//! This crate holds the configuration and the velocity math. It stays free of any
//! controller dialect, which the `lasercraft-gcode` and DSP crates own.

/// The machine motion limits.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotionConfig {
    /// The highest travel speed in millimetres per second.
    pub max_speed_mm_s: f64,
    /// The acceleration in millimetres per second squared.
    pub accel_mm_s2: f64,
    /// The acceleration-to-deceleration limit, the Klipper `ACCEL_TO_DECEL` value.
    pub accel_to_decel_mm_s2: f64,
}

impl MotionConfig {
    /// Clamp a requested feed rate to the configured maximum, never below zero.
    pub fn clamp_feed(&self, requested_mm_s: f64) -> f64 {
        requested_mm_s.clamp(0.0, self.max_speed_mm_s)
    }

    /// The highest speed reachable over a straight segment of `distance_mm`,
    /// starting from `entry_speed_mm_s`, bounded by acceleration and the maximum.
    ///
    /// Uses the constant-acceleration relation `v = sqrt(v0^2 + 2*a*d)`.
    pub fn max_reachable_speed(&self, distance_mm: f64, entry_speed_mm_s: f64) -> f64 {
        let distance = distance_mm.max(0.0);
        let entry = entry_speed_mm_s.max(0.0);
        let accel = self.accel_mm_s2.max(0.0);
        let reachable = (entry * entry + 2.0 * accel * distance).sqrt();
        reachable.min(self.max_speed_mm_s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> MotionConfig {
        MotionConfig {
            max_speed_mm_s: 100.0,
            accel_mm_s2: 1000.0,
            accel_to_decel_mm_s2: 500.0,
        }
    }

    #[test]
    fn feed_is_clamped_to_max() {
        assert_eq!(cfg().clamp_feed(250.0), 100.0);
        assert_eq!(cfg().clamp_feed(-5.0), 0.0);
        assert_eq!(cfg().clamp_feed(40.0), 40.0);
    }

    #[test]
    fn reachable_speed_respects_accel_and_cap() {
        // From rest over 2 mm at 1000 mm/s^2: sqrt(2*1000*2) = sqrt(4000) ~ 63.25.
        let v = cfg().max_reachable_speed(2.0, 0.0);
        assert!((v - 63.245).abs() < 0.01);
        // Over a long distance the cap applies.
        assert_eq!(cfg().max_reachable_speed(1000.0, 0.0), 100.0);
    }
}
