#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! WebAssembly bindings for the LaserCraft engine.
//!
//! The engine and the crates under it carry no filesystem, thread, time or
//! network calls, so the whole planning pipeline runs unchanged under
//! `wasm32-unknown-unknown`. This crate exposes it to JavaScript through
//! `wasm-bindgen`. Every entry point takes JSON in and returns a string or a
//! number out, so the browser never holds a Rust value across the boundary.
//!
//! The functions take untrusted input from the page, so each one parses with
//! `serde_json` and returns a [`JsError`] on bad input rather than trapping.

use lasercraft_engine::{
    FrameStyle, MaterialTestSpec, estimate_job_time, material_test, plan_frame_styled,
    plan_project, project_to_svg,
};
use lasercraft_gcode::GCodeDialect;
use lasercraft_laser_doc::Project;
use lasercraft_motion::MotionConfig;
use wasm_bindgen::prelude::*;

/// Map a controller name to its GCode dialect. An unknown name falls back to
/// plain GRBL, the most common flavour, so a typo still plans a usable program.
fn dialect(flavor: &str) -> GCodeDialect {
    match flavor {
        "grbl-m3" => GCodeDialect::grbl_m3(),
        "marlin" => GCodeDialect::marlin(),
        "smoothie" => GCodeDialect::smoothie(),
        _ => GCodeDialect::grbl(),
    }
}

fn parse<T: serde::de::DeserializeOwned>(json: &str, what: &str) -> Result<T, JsError> {
    serde_json::from_str(json).map_err(|e| JsError::new(&format!("invalid {what}: {e}")))
}

/// Plan a full GCode program for `project` under `motion`, for the named controller.
///
/// `flavor` is one of `grbl`, `grbl-m3`, `marlin` or `smoothie`.
#[wasm_bindgen]
pub fn generate_gcode(
    project_json: &str,
    motion_json: &str,
    flavor: &str,
) -> Result<String, JsError> {
    let project: Project = parse(project_json, "project")?;
    let motion: MotionConfig = parse(motion_json, "motion config")?;
    Ok(plan_project(&project, &motion, &dialect(flavor)))
}

/// Plan the framing pass the laser traces with the beam off, so the operator can
/// see where the job lands on the material.
///
/// `style` is `bounds`, `hull` or `contour`. An unknown value frames the bounds.
#[wasm_bindgen]
pub fn generate_frame(project_json: &str, style: &str, flavor: &str) -> Result<String, JsError> {
    let project: Project = parse(project_json, "project")?;
    let style = match style {
        "hull" => FrameStyle::Hull,
        "contour" => FrameStyle::Contour,
        _ => FrameStyle::Bounds,
    };
    Ok(plan_frame_styled(&project, style, &dialect(flavor)))
}

/// Render `project` to an SVG preview string for display in the page.
#[wasm_bindgen]
pub fn preview_svg(project_json: &str) -> Result<String, JsError> {
    let project: Project = parse(project_json, "project")?;
    Ok(project_to_svg(&project))
}

/// Estimate the job time in seconds for `project` under `motion`.
#[wasm_bindgen]
pub fn estimate_seconds(project_json: &str, motion_json: &str) -> Result<f64, JsError> {
    let project: Project = parse(project_json, "project")?;
    let motion: MotionConfig = parse(motion_json, "motion config")?;
    Ok(estimate_job_time(&project, &motion))
}

/// Build a material-test grid project from `spec` and return it as project JSON.
#[wasm_bindgen]
pub fn material_test_project(spec_json: &str) -> Result<String, JsError> {
    let spec: MaterialTestSpec = parse(spec_json, "material test spec")?;
    let project = material_test(&spec);
    serde_json::to_string(&project)
        .map_err(|e| JsError::new(&format!("could not serialise project: {e}")))
}
