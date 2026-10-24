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
    plan_galvo_lmc, plan_project, project_to_svg,
};
use lasercraft_galvo::GalvoParam;
use lasercraft_gcode::GCodeDialect;
use lasercraft_laser_doc::Project;
use lasercraft_motion::MotionConfig;
use wasm_bindgen::prelude::*;

/// Map a controller name to its GCode dialect. An unknown name falls back to
/// plain GRBL, the most common flavour, so a typo still plans a usable program.
fn dialect(flavor: &str) -> GCodeDialect {
    match flavor {
        "grbl-m3" => GCodeDialect::grbl_m3(),
        "grbl-lpc" => GCodeDialect::grbl_lpc(),
        "marlin" => GCodeDialect::marlin(),
        "smoothie" => GCodeDialect::smoothie(),
        "reprap" => GCodeDialect::reprap(),
        "linuxcnc" => GCodeDialect::linuxcnc(),
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

/// Plan a galvo project as BJJCZ LMC board bytes, ready for a WebUSB bulk write.
///
/// The field is a square of `field_mm` at `resolution_bits` (sixteen for most
/// boards). `mark_speed` and `power_ratio` are the board's raw values. The result
/// is the padded list buffer as a byte array, which the page writes to the galvo
/// controller's bulk endpoint.
#[wasm_bindgen]
pub fn generate_galvo_lmc(
    project_json: &str,
    field_mm: f64,
    resolution_bits: u32,
    frequency_khz: f64,
    jump_speed_mm_s: f64,
    mark_speed_mm_s: f64,
    power_percent: f64,
) -> Result<Vec<u8>, JsError> {
    let project: Project = parse(project_json, "project")?;
    let param = GalvoParam {
        field_size_mm: field_mm,
        resolution_bits,
    };
    let hz = (frequency_khz * 1000.0)
        .round()
        .clamp(0.0, f64::from(u32::MAX)) as u32;
    // Speeds reach the board in galvo units per millisecond, so a millimetre speed
    // is scaled by the device units per millimetre (the field maps onto the device
    // range). Power is the fiber current, full scale 0x0FFF.
    let span = field_mm.max(f64::MIN_POSITIVE);
    let galvos_per_mm = f64::from(param.device_max()) / span;
    let to_galvo = |mm_s: f64| (mm_s * galvos_per_mm / 1000.0).round().clamp(0.0, 65535.0) as u16;
    let jump = to_galvo(jump_speed_mm_s);
    let mark = to_galvo(mark_speed_mm_s);
    let power = ((power_percent / 100.0).clamp(0.0, 1.0) * 4095.0).round() as u16;
    Ok(plan_galvo_lmc(&project, &param, hz, jump, mark, power))
}

/// Union a set of closed polylines into their merged outline.
///
/// The input is a JSON array of polylines, each an array of `[x, y]` pairs. The
/// result is the same shape. This is the Weld operation, run on the shared
/// `craft-pathops` boolean.
#[wasm_bindgen]
pub fn weld(polylines_json: &str) -> Result<String, JsError> {
    let polylines: Vec<Vec<(f64, f64)>> = parse(polylines_json, "polylines")?;
    let out = lasercraft_geomops::weld(&polylines);
    serde_json::to_string(&out).map_err(|e| JsError::new(&format!("could not serialise: {e}")))
}

/// Combine two closed polylines with a boolean operation.
///
/// `op` is `difference` (a minus b), `intersect` (the overlap) or anything else
/// for `union`. Each input is an array of `[x, y]` pairs. The result is a JSON
/// array of result polylines. Run on the shared `craft-pathops` boolean.
#[wasm_bindgen]
pub fn boolean_op(a_json: &str, b_json: &str, op: &str) -> Result<String, JsError> {
    let a: Vec<(f64, f64)> = parse(a_json, "shape a")?;
    let b: Vec<(f64, f64)> = parse(b_json, "shape b")?;
    let out = match op {
        "difference" => lasercraft_geomops::difference(&a, &b),
        "intersect" => lasercraft_geomops::intersect(&a, &b),
        _ => lasercraft_geomops::weld(&[a, b]),
    };
    serde_json::to_string(&out).map_err(|e| JsError::new(&format!("could not serialise: {e}")))
}

/// Offset a closed outline inward into concentric rings.
///
/// The input is one polyline as an array of `[x, y]` pairs. The result is a JSON
/// array of ring polylines. This is the true offset fill, run on the shared
/// `craft-pathops` offset.
#[wasm_bindgen]
pub fn concentric(polyline_json: &str, interval: f64, max_rings: u32) -> Result<String, JsError> {
    let outline: Vec<(f64, f64)> = parse(polyline_json, "polyline")?;
    let out = lasercraft_geomops::concentric(&outline, interval, max_rings as usize);
    serde_json::to_string(&out).map_err(|e| JsError::new(&format!("could not serialise: {e}")))
}

/// Lay text out into single-stroke polylines for engraving or marking.
///
/// `height_mm` is the cap height. The baseline sits at `(x, y)` and text grows
/// upward. `gap_mm` is the extra space between letters. The result is a JSON array
/// of polylines, each an array of `[x, y]` pairs, which the page lays on the
/// active layer. The font is single-line, the right choice for a galvo or diode.
#[wasm_bindgen]
pub fn text_to_paths(
    text: &str,
    height_mm: f64,
    x: f64,
    y: f64,
    gap_mm: f64,
) -> Result<String, JsError> {
    let out = lasercraft_text::text_to_polylines(text, height_mm, x, y, gap_mm);
    serde_json::to_string(&out).map_err(|e| JsError::new(&format!("could not serialise: {e}")))
}

/// Offset a closed outline by one signed distance in millimetres.
///
/// A positive `delta` grows the outline outward, a negative one shrinks it inward.
/// The input is one polyline as an array of `[x, y]` pairs. The result is a JSON
/// array of offset polylines. This is Offset Shapes, run on the shared
/// `craft-pathops` offset.
#[wasm_bindgen]
pub fn offset(polyline_json: &str, delta: f64) -> Result<String, JsError> {
    let outline: Vec<(f64, f64)> = parse(polyline_json, "polyline")?;
    let out = lasercraft_geomops::offset(&outline, delta);
    serde_json::to_string(&out).map_err(|e| JsError::new(&format!("could not serialise: {e}")))
}

/// Read an ASCII DXF drawing into polylines for import.
///
/// The result is a JSON array of `{ points: [[x, y], ...], closed: bool }`, one per
/// supported entity (LINE, LWPOLYLINE, POLYLINE, CIRCLE, ARC). The page lays each
/// polyline on the active layer. Coordinates are drawing units, treated as mm.
#[wasm_bindgen]
pub fn import_dxf(text: &str) -> Result<String, JsError> {
    #[derive(serde::Serialize)]
    struct Poly {
        points: Vec<(f64, f64)>,
        closed: bool,
        layer: String,
    }
    let polys: Vec<Poly> = lasercraft_dxf::parse_dxf(text)
        .into_iter()
        .map(|p| Poly {
            points: p.points,
            closed: p.closed,
            layer: p.layer,
        })
        .collect();
    serde_json::to_string(&polys).map_err(|e| JsError::new(&format!("could not serialise: {e}")))
}

/// Write polylines out as an ASCII DXF drawing.
///
/// The input is a JSON array of `{ points: [[x, y], ...], closed: bool }`. The
/// result is DXF text the page offers as a download. This is the inverse of
/// [`import_dxf`], so a drawing round-trips through the editor.
#[wasm_bindgen]
pub fn export_dxf(polylines_json: &str) -> Result<String, JsError> {
    #[derive(serde::Deserialize)]
    struct Poly {
        points: Vec<(f64, f64)>,
        closed: bool,
        #[serde(default)]
        layer: String,
    }
    let polys: Vec<Poly> = parse(polylines_json, "polylines")?;
    let dxf: Vec<lasercraft_dxf::DxfPolyline> = polys
        .into_iter()
        .map(|p| lasercraft_dxf::DxfPolyline {
            points: p.points,
            closed: p.closed,
            layer: if p.layer.is_empty() {
                "0".to_string()
            } else {
                p.layer
            },
        })
        .collect();
    Ok(lasercraft_dxf::write_dxf(&dxf))
}

/// Parse one SVG path `d` attribute into cubic bezier subpaths.
///
/// The result is a JSON array of paths, each `{ start, segments, closed }` in the
/// document's shape form, so curves import as true beziers the node tool can edit.
/// Bad path data returns an error. The page reads the layer from the element.
#[wasm_bindgen]
pub fn svg_path_to_paths(d: &str) -> Result<String, JsError> {
    let paths = lasercraft_svg::parse_path_data(d)
        .map_err(|e| JsError::new(&format!("invalid path data: {e}")))?;
    serde_json::to_string(&paths).map_err(|e| JsError::new(&format!("could not serialise: {e}")))
}

/// Lay text out into filled glyph outlines from a font file.
///
/// `font` is the raw bytes of a TrueType or OpenType font, passed from the page. The
/// result is a JSON array of closed contours, each an array of `[x, y]` pairs in
/// millimetres, which the page lays on the active layer. This is filled text, the
/// companion to the single-stroke `text_to_paths`.
#[wasm_bindgen]
pub fn text_outline_paths(
    font: &[u8],
    text: &str,
    height_mm: f64,
    x: f64,
    y: f64,
) -> Result<String, JsError> {
    let out = lasercraft_text::glyph_outlines(font, text, height_mm, x, y);
    serde_json::to_string(&out).map_err(|e| JsError::new(&format!("could not serialise: {e}")))
}

/// Trace a one-bit image mask into closed vector contours.
///
/// `mask` is one byte per pixel, row-major, non-zero for a mark pixel. The result
/// is a JSON array of `{ points: [[x, y], ...], closed: bool }` in pixel
/// coordinates, which the page scales onto the bed. `speckle` drops contours below
/// that pixel area and `smoothing` sets the corner tolerance.
#[wasm_bindgen]
pub fn trace_image(
    mask: &[u8],
    width: u32,
    height: u32,
    speckle: u32,
    smoothing: f64,
) -> Result<String, JsError> {
    use lasercraft_trace::{Bitmap, BoundaryTracer, TraceParams, Tracer};
    let expected = (width as usize).saturating_mul(height as usize);
    if mask.len() != expected {
        return Err(JsError::new("mask size does not match width times height"));
    }
    let bitmap = Bitmap {
        width,
        height,
        pixels: mask.iter().map(|&b| b != 0).collect(),
    };
    let params = TraceParams {
        threshold: 128,
        speckle_suppression: speckle,
        corner_smoothing: smoothing,
    };
    #[derive(serde::Serialize)]
    struct Poly {
        points: Vec<(f64, f64)>,
        closed: bool,
    }
    let out: Vec<Poly> = BoundaryTracer
        .trace(&bitmap, &params)
        .into_iter()
        .map(|p| Poly {
            points: p.points.into_iter().map(|q| (q.x, q.y)).collect(),
            closed: p.closed,
        })
        .collect();
    serde_json::to_string(&out).map_err(|e| JsError::new(&format!("could not serialise: {e}")))
}

/// Build a material-test grid project from `spec` and return it as project JSON.
#[wasm_bindgen]
pub fn material_test_project(spec_json: &str) -> Result<String, JsError> {
    let spec: MaterialTestSpec = parse(spec_json, "material test spec")?;
    let project = material_test(&spec);
    serde_json::to_string(&project)
        .map_err(|e| JsError::new(&format!("could not serialise project: {e}")))
}
