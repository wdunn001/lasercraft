#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! The LaserCraft command-line interface.
//!
//! A thin headless front end over `lasercraft-engine`, matching the Craft suite
//! convention of shipping an `<app>-cli` alongside the GUI. For now it generates a
//! line-fill job and prints the controller GCode, which exercises the whole
//! document-to-output pipeline from a terminal.

use clap::{Parser, Subcommand, ValueEnum};
use lasercraft_engine::plan_line_fill;
use lasercraft_gcode::Grbl;
use lasercraft_laser_doc::CutSetting;
use lasercraft_motion::MotionConfig;
use lasercraft_toolpath::BBox;

/// LaserCraft command-line interface.
#[derive(Debug, Parser)]
#[command(name = "lasercraft-cli", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// The controller dialect to emit.
#[derive(Debug, Clone, Copy, ValueEnum)]
enum Flavor {
    /// GRBL and compatible controllers.
    Grbl,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate a line-fill job for a rectangular region and print the GCode.
    Linefill {
        /// Region width in millimetres.
        #[arg(long, default_value_t = 50.0)]
        width: f64,
        /// Region height in millimetres.
        #[arg(long, default_value_t = 50.0)]
        height: f64,
        /// Scan line spacing in millimetres.
        #[arg(long, default_value_t = 0.1)]
        interval: f64,
        /// Travel speed in millimetres per second.
        #[arg(long, default_value_t = 100.0)]
        speed: f64,
        /// Laser power percentage.
        #[arg(long, default_value_t = 60.0)]
        power: f64,
        /// Machine maximum speed in millimetres per second.
        #[arg(long, default_value_t = 300.0)]
        max_speed: f64,
        /// Controller dialect.
        #[arg(long, value_enum, default_value_t = Flavor::Grbl)]
        flavor: Flavor,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Linefill {
            width,
            height,
            interval,
            speed,
            power,
            max_speed,
            flavor,
        } => {
            let bbox = BBox {
                min_x: 0.0,
                min_y: 0.0,
                max_x: width,
                max_y: height,
            };
            let setting = CutSetting {
                index: 0,
                name: "Fill".to_string(),
                speed_mm_s: speed,
                power_min: 0.0,
                power_max: power,
                num_passes: 1,
                air_assist: true,
            };
            let motion = MotionConfig {
                max_speed_mm_s: max_speed,
                accel_mm_s2: 1000.0,
                accel_to_decel_mm_s2: 500.0,
            };
            let gcode = match flavor {
                Flavor::Grbl => plan_line_fill(&bbox, &setting, &motion, interval, &Grbl::new()),
            };
            print!("{gcode}");
        }
    }
}
