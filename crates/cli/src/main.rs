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
//! convention of shipping an `<app>-cli` alongside the GUI. It can generate a
//! line-fill job for a region or load a saved project and emit its cutting GCode,
//! which exercises the document, persistence, engine, and dialect crates together.

use clap::{Parser, Subcommand, ValueEnum};
use lasercraft_engine::{
    estimate_job_time, plan_frame, plan_line_fill, plan_project, plan_raster, project_to_svg,
};
use lasercraft_gcode::{Dialect, GCodeDialect};
use lasercraft_laser_doc::CutSetting;
use lasercraft_motion::MotionConfig;
use lasercraft_raster::{GrayImage, RasterParams};
use lasercraft_toolpath::BBox;
use std::path::PathBuf;

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
    /// GRBL constant-power (M3) mode, for 1.1e and earlier.
    GrblM3,
    /// Marlin firmware.
    Marlin,
    /// Smoothieware.
    Smoothie,
}

impl Flavor {
    fn dialect(self) -> Box<dyn Dialect> {
        match self {
            Flavor::Grbl => Box::new(GCodeDialect::grbl()),
            Flavor::GrblM3 => Box::new(GCodeDialect::grbl_m3()),
            Flavor::Marlin => Box::new(GCodeDialect::marlin()),
            Flavor::Smoothie => Box::new(GCodeDialect::smoothie()),
        }
    }
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
    /// Load a saved project (JSON) and print the GCode that cuts its shapes.
    Cut {
        /// Path to the project file.
        #[arg(long)]
        project: PathBuf,
        /// Machine maximum speed in millimetres per second.
        #[arg(long, default_value_t = 300.0)]
        max_speed: f64,
        /// Controller dialect.
        #[arg(long, value_enum, default_value_t = Flavor::Grbl)]
        flavor: Flavor,
    },
    /// Load a saved project and print the estimated cutting time.
    Estimate {
        /// Path to the project file.
        #[arg(long)]
        project: PathBuf,
        /// Machine maximum speed in millimetres per second.
        #[arg(long, default_value_t = 300.0)]
        max_speed: f64,
    },
    /// Load a saved project and print an SVG preview of it.
    Preview {
        /// Path to the project file.
        #[arg(long)]
        project: PathBuf,
    },
    /// Load a saved project and print a framing pass tracing its bounds.
    Frame {
        /// Path to the project file.
        #[arg(long)]
        project: PathBuf,
        /// Controller dialect.
        #[arg(long, value_enum, default_value_t = Flavor::Grbl)]
        flavor: Flavor,
    },
    /// Load a binary PGM image and print the GCode that raster-engraves it.
    Raster {
        /// Path to the PGM (P5) image.
        #[arg(long)]
        image: PathBuf,
        /// Physical size of one pixel in millimetres.
        #[arg(long, default_value_t = 0.1)]
        pixel: f64,
        /// Power percentage for a fully dark pixel.
        #[arg(long, default_value_t = 60.0)]
        power: f64,
        /// Samples at or below this value are skipped as blank.
        #[arg(long, default_value_t = 20)]
        threshold: u8,
        /// Engrave speed in millimetres per second.
        #[arg(long, default_value_t = 150.0)]
        speed: f64,
        /// Machine maximum speed in millimetres per second.
        #[arg(long, default_value_t = 300.0)]
        max_speed: f64,
        /// Controller dialect.
        #[arg(long, value_enum, default_value_t = Flavor::Grbl)]
        flavor: Flavor,
    },
}

fn main() -> std::process::ExitCode {
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
            let motion = motion_for(max_speed);
            let gcode = plan_line_fill(
                &bbox,
                &setting,
                &motion,
                interval,
                flavor.dialect().as_ref(),
            );
            print!("{gcode}");
            std::process::ExitCode::SUCCESS
        }
        Commands::Cut {
            project,
            max_speed,
            flavor,
        } => match lasercraft_persist::load_project(&project) {
            Ok(doc) => {
                let motion = motion_for(max_speed);
                let gcode = plan_project(&doc, &motion, flavor.dialect().as_ref());
                print!("{gcode}");
                std::process::ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("could not load {}: {error}", project.display());
                std::process::ExitCode::FAILURE
            }
        },
        Commands::Estimate { project, max_speed } => {
            match lasercraft_persist::load_project(&project) {
                Ok(doc) => {
                    let seconds = estimate_job_time(&doc, &motion_for(max_speed));
                    let minutes = (seconds / 60.0) as u64;
                    let rem = seconds - (minutes as f64) * 60.0;
                    println!("Estimated cutting time: {minutes}m {rem:.1}s ({seconds:.1}s)");
                    std::process::ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("could not load {}: {error}", project.display());
                    std::process::ExitCode::FAILURE
                }
            }
        }
        Commands::Preview { project } => match lasercraft_persist::load_project(&project) {
            Ok(doc) => {
                print!("{}", project_to_svg(&doc));
                std::process::ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("could not load {}: {error}", project.display());
                std::process::ExitCode::FAILURE
            }
        },
        Commands::Frame { project, flavor } => match lasercraft_persist::load_project(&project) {
            Ok(doc) => {
                print!("{}", plan_frame(&doc, flavor.dialect().as_ref()));
                std::process::ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("could not load {}: {error}", project.display());
                std::process::ExitCode::FAILURE
            }
        },
        Commands::Raster {
            image,
            pixel,
            power,
            threshold,
            speed,
            max_speed,
            flavor,
        } => {
            let bytes = match std::fs::read(&image) {
                Ok(bytes) => bytes,
                Err(error) => {
                    eprintln!("could not read {}: {error}", image.display());
                    return std::process::ExitCode::FAILURE;
                }
            };
            let gray = match GrayImage::from_pgm(&bytes) {
                Ok(gray) => gray,
                Err(error) => {
                    eprintln!("could not parse {}: {error}", image.display());
                    return std::process::ExitCode::FAILURE;
                }
            };
            let params = RasterParams {
                pixel_mm: pixel,
                power_max: power,
                threshold,
                speed_mm_s: speed,
            };
            let motion = motion_for(max_speed);
            let gcode = plan_raster(&gray, &params, &motion, flavor.dialect().as_ref());
            print!("{gcode}");
            std::process::ExitCode::SUCCESS
        }
    }
}

fn motion_for(max_speed_mm_s: f64) -> MotionConfig {
    MotionConfig {
        max_speed_mm_s,
        accel_mm_s2: 1000.0,
        accel_to_decel_mm_s2: 500.0,
    }
}
