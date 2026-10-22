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

    /// Estimate the time in seconds to travel a straight segment of `distance_mm`
    /// at a target `feed_mm_s`, starting and stopping at rest.
    ///
    /// This models the real trapezoidal velocity profile: accelerate to the feed,
    /// cruise, then decelerate. Short moves that cannot reach the feed use the
    /// triangular profile instead. Assuming rest at both ends is a conservative
    /// estimate that ignores junction carry-over, which is exactly why LightBurn's
    /// naive distance-over-feed estimate reads far too optimistic on detailed jobs.
    pub fn move_time(&self, distance_mm: f64, feed_mm_s: f64) -> f64 {
        let distance = distance_mm.max(0.0);
        let accel = self.accel_mm_s2.max(f64::MIN_POSITIVE);
        let feed = self.clamp_feed(feed_mm_s).max(f64::MIN_POSITIVE);
        if distance == 0.0 {
            return 0.0;
        }
        // Distance to accelerate from rest to the feed, and the same to stop.
        let ramp_distance = feed * feed / (2.0 * accel);
        if 2.0 * ramp_distance <= distance {
            // Trapezoid: ramp up, cruise, ramp down.
            let ramp_time = 2.0 * (feed / accel);
            let cruise_time = (distance - 2.0 * ramp_distance) / feed;
            ramp_time + cruise_time
        } else {
            // Triangle: accelerate to a peak below the feed, then decelerate.
            let peak = (accel * distance).sqrt();
            2.0 * peak / accel
        }
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

    #[test]
    fn move_time_uses_a_trapezoid_when_the_feed_is_reached() {
        // 10 mm at 50 mm/s, accel 1000: ramp 1.25 mm each end, so a trapezoid.
        // t = 2*(50/1000) + (10 - 2.5)/50 = 0.1 + 0.15 = 0.25 s.
        let t = cfg().move_time(10.0, 50.0);
        assert!((t - 0.25).abs() < 1e-9, "got {t}");
    }

    #[test]
    fn move_time_uses_a_triangle_for_short_moves() {
        // 1 mm at 50 mm/s, accel 1000: cannot reach 50, peak = sqrt(1000*1) ~ 31.6,
        // t = 2*peak/accel ~ 0.06325 s.
        let t = cfg().move_time(1.0, 50.0);
        assert!((t - 0.063245).abs() < 1e-4, "got {t}");
    }
}
