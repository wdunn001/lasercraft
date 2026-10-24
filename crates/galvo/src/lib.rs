#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]
//! Galvo and fiber laser support for LaserCraft.
//!
//! Reverse engineering of `executor.dll` found a `CalibFunc` class doing galvo
//! field correction, with `logicPtToDevPt` mapping a logical point in the field to
//! device coordinates, a `GalvoParam` carrying the field, and `setResolution`. See
//! `docs/rea-findings.md`. This crate reproduces that mapping. Grid correction from
//! a measured calibration file lands on top of it later.

use lasercraft_device::{Descriptor, Protocol, TransportKind};

/// The galvo field parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GalvoParam {
    /// The square field side length in millimetres.
    pub field_size_mm: f64,
    /// The device coordinate resolution in bits, as set by `setResolution`.
    pub resolution_bits: u32,
}

impl GalvoParam {
    /// The maximum device coordinate value for the configured resolution.
    pub fn device_max(&self) -> u32 {
        let bits = self.resolution_bits.clamp(1, 31);
        (1u32 << bits) - 1
    }

    /// Map a point in millimetres to device coordinates.
    ///
    /// The editor uses a corner-origin work area, so the millimetre range
    /// `[0, field]` maps linearly onto `[0, device_max]`: a point at the field
    /// centre lands at the device centre and the field corner at device zero.
    /// Points outside the field are clamped. The field size must equal the work
    /// area the design is drawn on, or the design collapses toward one corner.
    pub fn logic_to_device(&self, x_mm: f64, y_mm: f64) -> (i32, i32) {
        (self.axis_to_device(x_mm), self.axis_to_device(y_mm))
    }

    fn axis_to_device(&self, mm: f64) -> i32 {
        let span = self.field_size_mm.max(f64::MIN_POSITIVE);
        let max = f64::from(self.device_max());
        let normalized = (mm / span).clamp(0.0, 1.0);
        (normalized * max).round() as i32
    }
}

/// Build the descriptor for a galvo controller.
pub fn galvo_descriptor() -> Descriptor {
    Descriptor {
        id: "galvo".to_string(),
        display_name: "Galvo / Fiber".to_string(),
        transport: TransportKind::Usb,
        gcode_flavor: None,
    }
}

/// A galvo controller protocol instance.
#[derive(Debug, Clone, PartialEq)]
pub struct GalvoProtocol {
    param: GalvoParam,
    descriptor: Descriptor,
}

impl GalvoProtocol {
    /// A galvo protocol with the given field parameters.
    pub fn new(param: GalvoParam) -> Self {
        Self {
            param,
            descriptor: galvo_descriptor(),
        }
    }

    /// The field parameters.
    pub fn param(&self) -> GalvoParam {
        self.param
    }
}

impl Protocol for GalvoProtocol {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

/// One galvo marking instruction in device coordinates.
///
/// A galvo controller does not speak GCode. It consumes a list of jumps and marks
/// over the mirror field, with power and the MOPA pulse frequency set between them.
/// This is the device-neutral marking model. The board-specific byte encoding
/// (the JCZ or LMC list format behind EZCAD2) serializes these later, the same way
/// a GCode dialect serializes the neutral ops.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GalvoCommand {
    /// Move with the laser off to a device point.
    Jump {
        /// Device X.
        x: i32,
        /// Device Y.
        y: i32,
    },
    /// Move with the laser on to a device point.
    Mark {
        /// Device X.
        x: i32,
        /// Device Y.
        y: i32,
    },
    /// Set the marking power as a percentage.
    SetPower {
        /// Power percent from 0.0 to 100.0.
        percent: f64,
    },
    /// Set the MOPA pulse frequency in hertz.
    SetFrequency {
        /// Frequency in hertz.
        hz: u32,
    },
}

/// Convert a logical polyline in millimetres into galvo marking commands.
///
/// Power and frequency are set first, then a jump to the first point, then a mark
/// to each subsequent point. A closed polyline marks back to its start. Points are
/// mapped through the field correction in `param`. Fewer than two points yields
/// only the power and frequency commands.
pub fn mark_polyline(
    param: &GalvoParam,
    points: &[(f64, f64)],
    closed: bool,
    power_percent: f64,
    frequency_hz: u32,
) -> Vec<GalvoCommand> {
    let mut commands = vec![
        GalvoCommand::SetPower {
            percent: power_percent.clamp(0.0, 100.0),
        },
        GalvoCommand::SetFrequency { hz: frequency_hz },
    ];
    let Some((first_x, first_y)) = points.first().copied() else {
        return commands;
    };
    if points.len() < 2 {
        return commands;
    }
    let (jx, jy) = param.logic_to_device(first_x, first_y);
    commands.push(GalvoCommand::Jump { x: jx, y: jy });
    for &(x, y) in &points[1..] {
        let (dx, dy) = param.logic_to_device(x, y);
        commands.push(GalvoCommand::Mark { x: dx, y: dy });
    }
    if closed {
        commands.push(GalvoCommand::Mark { x: jx, y: jy });
    }
    commands
}

/// One constant-power horizontal span to mark, in millimetres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MarkRun {
    /// The row position.
    pub y_mm: f64,
    /// Where the span starts.
    pub x_start_mm: f64,
    /// Where the span ends.
    pub x_end_mm: f64,
    /// The marking power percentage.
    pub power_percent: f64,
}

/// Convert constant-power scan runs into galvo marking commands.
///
/// This is the galvo analogue of raster engraving: each run jumps to its start
/// with the beam off, then marks across at the run power. Power is set only when it
/// changes between runs, so a flat image does not re-send it on every line.
pub fn mark_scanlines(
    param: &GalvoParam,
    runs: &[MarkRun],
    frequency_hz: u32,
) -> Vec<GalvoCommand> {
    let mut commands = vec![GalvoCommand::SetFrequency { hz: frequency_hz }];
    let mut last_power: Option<i64> = None;
    for run in runs {
        let quantized = run.power_percent.round() as i64;
        if last_power != Some(quantized) {
            commands.push(GalvoCommand::SetPower {
                percent: run.power_percent.clamp(0.0, 100.0),
            });
            last_power = Some(quantized);
        }
        let (sx, sy) = param.logic_to_device(run.x_start_mm, run.y_mm);
        let (ex, ey) = param.logic_to_device(run.x_end_mm, run.y_mm);
        commands.push(GalvoCommand::Jump { x: sx, y: sy });
        commands.push(GalvoCommand::Mark { x: ex, y: ey });
    }
    commands
}

/// A galvo lens correction table parsed from an EZCAD `JCZ_COR_2_1` `.cor` file.
///
/// The file is a signature, a short header carrying a scale float, then a square
/// grid of per-cell `(dx, dy)` device-unit offsets that straighten the lens
/// distortion across the field. Sampling bilinearly between cells gives the
/// correction to add at any field position. Format confirmed against a real
/// ComMarker B6 `jcz15.cor` (65x65 grid, 40-byte header).
#[derive(Debug, Clone, PartialEq)]
pub struct CorrectionTable {
    /// The grid side length, for example 65.
    pub size: usize,
    /// The scale factor from the header.
    pub scale: f32,
    cells: Vec<(i32, i32)>,
}

const COR_SIGNATURE: &[u8] = b"JCZ_COR_2_1";
const COR_HEADER_LEN: usize = 40;

impl CorrectionTable {
    /// Parse a `.cor` file. Errors on a bad signature or a non-square grid.
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < COR_HEADER_LEN {
            return Err("cor file too short");
        }
        if bytes.get(..COR_SIGNATURE.len()) != Some(COR_SIGNATURE) {
            return Err("not a JCZ_COR correction file");
        }
        let data_len = bytes.len() - COR_HEADER_LEN;
        if !data_len.is_multiple_of(8) {
            return Err("cor data is not whole (dx, dy) pairs");
        }
        let points = data_len / 8;
        let size = (points as f64).sqrt().round() as usize;
        // checked_mul guards the 32-bit usize on wasm32, where size * size could wrap.
        if size.checked_mul(size) != Some(points) {
            return Err("cor grid is not square");
        }
        // Bilinear sampling needs at least a 2x2 grid, so a degenerate table is rejected.
        if size < 2 {
            return Err("cor grid too small");
        }
        let scale_bytes: [u8; 4] = bytes
            .get(32..36)
            .and_then(|s| s.try_into().ok())
            .ok_or("cor header truncated")?;
        let scale = f32::from_le_bytes(scale_bytes);
        let mut cells = Vec::with_capacity(points);
        for i in 0..points {
            let o = COR_HEADER_LEN + i * 8;
            let dx = read_i32(bytes, o)?;
            let dy = read_i32(bytes, o + 4)?;
            cells.push((dx, dy));
        }
        Ok(Self { size, scale, cells })
    }

    fn cell(&self, gx: usize, gy: usize) -> (i32, i32) {
        self.cells
            .get(gy * self.size + gx)
            .copied()
            .unwrap_or((0, 0))
    }

    /// Bilinearly sample the `(dx, dy)` correction for normalized field coordinates,
    /// where `u` and `v` run from 0.0 to 1.0 across the field. Out-of-range inputs
    /// clamp to the edge.
    pub fn correction_at(&self, u: f64, v: f64) -> (f64, f64) {
        if self.size < 2 {
            return (0.0, 0.0);
        }
        let max_index = (self.size - 1) as f64;
        let fx = u.clamp(0.0, 1.0) * max_index;
        let fy = v.clamp(0.0, 1.0) * max_index;
        let x0 = fx.floor() as usize;
        let y0 = fy.floor() as usize;
        let x1 = (x0 + 1).min(self.size - 1);
        let y1 = (y0 + 1).min(self.size - 1);
        let tx = fx - x0 as f64;
        let ty = fy - y0 as f64;
        let lerp = |a: f64, b: f64, t: f64| a + (b - a) * t;
        let (c00, c10, c01, c11) = (
            self.cell(x0, y0),
            self.cell(x1, y0),
            self.cell(x0, y1),
            self.cell(x1, y1),
        );
        let dx = lerp(
            lerp(f64::from(c00.0), f64::from(c10.0), tx),
            lerp(f64::from(c01.0), f64::from(c11.0), tx),
            ty,
        );
        let dy = lerp(
            lerp(f64::from(c00.1), f64::from(c10.1), tx),
            lerp(f64::from(c01.1), f64::from(c11.1), tx),
            ty,
        );
        (dx, dy)
    }
}

fn read_i32(bytes: &[u8], offset: usize) -> Result<i32, &'static str> {
    bytes
        .get(offset..offset + 4)
        .and_then(|s| s.try_into().ok())
        .map(i32::from_le_bytes)
        .ok_or("cor data truncated")
}

/// The BJJCZ LMC board list encoding behind EZCAD2 and JCZFiber.
///
/// A GCode dialect serialises the neutral GCode ops; this module is the same seam
/// for the galvo board. It turns [`GalvoCommand`] into the LMC list-buffer bytes a
/// BJJCZ controller executes.
///
/// The format is clean-room from the public community documentation of the board,
/// the `meerk40t/galvoplotter` and Balor projects, not from any LightBurn or JCZ
/// code. A command is six little-endian `u16`: an opcode and five parameters, so
/// twelve bytes. Commands accumulate into a list chunk of `0xC00` bytes, which is
/// 256 commands, padded with end-of-list opcodes. Field coordinates are `0..=0xFFFF`
/// with `0x8000` at the centre, which is what [`GalvoParam::logic_to_device`]
/// already produces at sixteen-bit resolution.
///
/// The opcodes and the packet layout are verified against the public sources. The
/// power-ratio scale is machine-dependent, so [`power_ratio_from_percent`] is a
/// provisional linear map that a real machine's calibration refines. Nothing here
/// is confirmed against hardware yet.
pub mod lmc {
    use super::GalvoCommand;

    /// The BJJCZ USB vendor id.
    pub const USB_VENDOR: u16 = 0x9588;
    /// The BJJCZ USB product id.
    pub const USB_PRODUCT: u16 = 0x9899;
    /// The bulk OUT endpoint the list is written to.
    pub const WRITE_ENDPOINT: u8 = 0x02;
    /// The bulk IN endpoint status is read from.
    pub const READ_ENDPOINT: u8 = 0x88;

    /// Move with the laser off to a field point.
    pub const LIST_JUMP_TO: u16 = 0x8001;
    /// End of a list, also the padding opcode.
    pub const LIST_END_OF_LIST: u16 = 0x8002;
    /// A dwell written into the list (10 us units). Opens a marking list.
    pub const LIST_DELAY: u16 = 0x8004;
    /// Move with the laser on to a field point.
    pub const LIST_MARK_TO: u16 = 0x8005;
    /// Set the jump (travel) speed.
    pub const LIST_JUMP_SPEED: u16 = 0x8006;
    /// Write the output port from inside the list. The board gates the fiber on
    /// the laser-enable bit here, so a mark list must set it or nothing fires.
    pub const LIST_WRITE_PORT: u16 = 0x8011;
    /// Set the fiber marking current, the power control a MOPA source actually
    /// honours (the power-ratio word 0x800B is ignored on this controller).
    pub const LIST_MARK_CURRENT: u16 = 0x8012;
    /// Set the laser-on delay (Laser On TC).
    pub const LIST_LASER_ON_DELAY: u16 = 0x8007;
    /// Set the laser-off delay (Laser Off TC).
    pub const LIST_LASER_OFF_DELAY: u16 = 0x8008;
    /// Set the power ratio on machines that take it over 0x800A.
    pub const LIST_MARK_POWER_RATIO: u16 = 0x800B;
    /// Set the marking speed.
    pub const LIST_MARK_SPEED: u16 = 0x800C;
    /// Set the jump delay.
    pub const LIST_JUMP_DELAY: u16 = 0x800D;
    /// Set the polygon (corner) delay.
    pub const LIST_POLYGON_DELAY: u16 = 0x800F;
    /// Set the Q-switch period, the fiber laser's inverse frequency.
    pub const LIST_QSWITCH_PERIOD: u16 = 0x801B;
    /// Start-of-list marker. EZCAD opens every mark list with this before any
    /// speed or geometry command, and the board needs it to arm the list for
    /// execution. Recovered from the EZCAD marking library (balor cross-check).
    pub const LIST_READY_MARK: u16 = 0x8051;

    /// Bytes in one command packet.
    pub const PACKET_LEN: usize = 12;
    /// Bytes in one list chunk, which is 256 commands.
    pub const LIST_CHUNK_LEN: usize = 0xC00;
    /// The base constant that converts a fiber frequency in kHz to a Q-switch period.
    const QSWITCH_BASE: f64 = 20000.0;

    /// Encode one opcode and its five parameters into a twelve-byte packet.
    pub fn encode(command: u16, params: [u16; 5]) -> [u8; PACKET_LEN] {
        let mut packet = [0u8; PACKET_LEN];
        packet[0..2].copy_from_slice(&command.to_le_bytes());
        for (i, value) in params.iter().enumerate() {
            let offset = 2 + i * 2;
            packet[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
        }
        packet
    }

    /// The Q-switch period for a fiber marking frequency in kHz.
    ///
    /// The period is `20000 / frequency`, clamped to a `u16`. A frequency at or
    /// below zero yields zero.
    pub fn qswitch_period(frequency_khz: f64) -> u16 {
        if frequency_khz <= 0.0 {
            return 0;
        }
        (QSWITCH_BASE / frequency_khz)
            .round()
            .clamp(0.0, f64::from(u16::MAX)) as u16
    }

    /// A provisional map from a power percentage to the list power ratio.
    ///
    /// The ratio runs `0..=0xFFF`, a common twelve-bit full scale. The true scale
    /// depends on the laser and its current source, so a machine's calibration
    /// replaces this. The value is clamped to the percentage range first.
    pub fn power_ratio_from_percent(percent: f64) -> u16 {
        let fraction = (percent / 100.0).clamp(0.0, 1.0);
        (fraction * f64::from(0x0FFFu16)).round() as u16
    }

    /// A signed microsecond delay as `(magnitude, sign)`, where a negative delay
    /// sets `0x8000` in the sign slot, as the board expects for an advanced delay.
    fn delay_params(delay_us: f64) -> [u16; 5] {
        let magnitude = delay_us.abs().round().clamp(0.0, f64::from(u16::MAX)) as u16;
        let sign = if delay_us < 0.0 { 0x8000 } else { 0x0000 };
        [magnitude, sign, 0, 0, 0]
    }

    fn clamp_coord(value: i32) -> u16 {
        value.clamp(0, i32::from(u16::MAX)) as u16
    }

    /// A growing LMC list program.
    #[derive(Debug, Default, Clone)]
    pub struct ListBuffer {
        bytes: Vec<u8>,
        count: usize,
    }

    impl ListBuffer {
        /// An empty list.
        pub fn new() -> Self {
            Self::default()
        }

        /// Append one command.
        pub fn push(&mut self, command: u16, params: [u16; 5]) {
            self.bytes.extend_from_slice(&encode(command, params));
            self.count += 1;
        }

        /// Jump with the laser off to a field point. The fourth word carries the
        /// straight-line distance from the previous point, which the board uses to
        /// time the move; it is clamped to the 16-bit range.
        pub fn jump_to(&mut self, x: u16, y: u16, distance: u16) {
            self.push(LIST_JUMP_TO, [x, y, 0, distance, 0]);
        }

        /// Mark with the laser on to a field point, with the point-to-point
        /// distance in the fourth word, as the board expects.
        pub fn mark_to(&mut self, x: u16, y: u16, distance: u16) {
            self.push(LIST_MARK_TO, [x, y, 0, distance, 0]);
        }

        /// Open the list with the start-of-list marker the board arms on.
        pub fn list_ready(&mut self) {
            self.push(LIST_READY_MARK, [0, 0, 0, 0, 0]);
        }

        /// A dwell at the head of the list, in 10 us units.
        pub fn delay(&mut self, ticks: u16) {
            self.push(LIST_DELAY, [ticks, 0, 0, 0, 0]);
        }

        /// Write the output-port bits from inside the list (laser enable gate).
        pub fn write_port(&mut self, bits: u16) {
            self.push(LIST_WRITE_PORT, [bits, 0, 0, 0, 0]);
        }

        /// Set the fiber marking current (power on a MOPA source).
        pub fn mark_current(&mut self, current: u16) {
            self.push(LIST_MARK_CURRENT, [current, 0, 0, 0, 0]);
        }

        /// Set the marking speed.
        pub fn mark_speed(&mut self, speed: u16) {
            self.push(LIST_MARK_SPEED, [speed, 0, 0, 0, 0]);
        }

        /// Set the jump speed.
        pub fn jump_speed(&mut self, speed: u16) {
            self.push(LIST_JUMP_SPEED, [speed, 0, 0, 0, 0]);
        }

        /// Set the power ratio.
        pub fn mark_power_ratio(&mut self, ratio: u16) {
            self.push(LIST_MARK_POWER_RATIO, [ratio, 0, 0, 0, 0]);
        }

        /// Set the fiber Q-switch period.
        pub fn qswitch_period(&mut self, period: u16) {
            self.push(LIST_QSWITCH_PERIOD, [period, 0, 0, 0, 0]);
        }

        /// Set the laser-on delay in microseconds.
        pub fn laser_on_delay(&mut self, delay_us: f64) {
            self.push(LIST_LASER_ON_DELAY, delay_params(delay_us));
        }

        /// Set the laser-off delay in microseconds.
        pub fn laser_off_delay(&mut self, delay_us: f64) {
            self.push(LIST_LASER_OFF_DELAY, delay_params(delay_us));
        }

        /// Set the jump delay in microseconds.
        pub fn jump_delay(&mut self, delay_us: f64) {
            self.push(LIST_JUMP_DELAY, delay_params(delay_us));
        }

        /// Set the polygon (corner) delay in microseconds.
        pub fn polygon_delay(&mut self, delay_us: f64) {
            self.push(LIST_POLYGON_DELAY, delay_params(delay_us));
        }

        /// The number of commands written, before padding.
        pub fn len(&self) -> usize {
            self.count
        }

        /// Whether no command has been written.
        pub fn is_empty(&self) -> bool {
            self.count == 0
        }

        /// The finished buffer: an end-of-list, then padding with end-of-list
        /// opcodes up to a whole `0xC00` chunk, which is what the board reads.
        pub fn finish(mut self) -> Vec<u8> {
            self.push(LIST_END_OF_LIST, [0; 5]);
            while !self.bytes.len().is_multiple_of(LIST_CHUNK_LEN) {
                self.bytes
                    .extend_from_slice(&encode(LIST_END_OF_LIST, [0; 5]));
            }
            self.bytes
        }
    }

    /// Compile device-space galvo commands into an LMC list program.
    ///
    /// The marking speed and the power ratio open the list. Each [`GalvoCommand`]
    /// then appends its list command. The result is padded to a whole chunk and is
    /// ready for the bulk write to [`WRITE_ENDPOINT`].
    pub fn compile(
        commands: &[GalvoCommand],
        jump_speed: u16,
        mark_speed: u16,
        power: u16,
    ) -> Vec<u8> {
        let mut list = ListBuffer::new();
        let mut last: Option<(u16, u16)> = None;
        // The opening sequence a working mark list carries on this controller: the
        // ready marker, a short dwell, the laser-enable port bit written into the
        // list, the travel and mark speeds, the fiber current (power), and the
        // on/off and polygon delays. The Q-switch (frequency) is set by the first
        // SetFrequency command below.
        list.list_ready();
        list.delay(0x0320);
        list.write_port(0x0001);
        list.jump_speed(jump_speed);
        list.mark_speed(mark_speed);
        list.mark_current(power);
        list.laser_on_delay(100.0);
        list.laser_off_delay(100.0);
        list.polygon_delay(10.0);
        for command in commands {
            match *command {
                GalvoCommand::Jump { x, y } => {
                    let (cx, cy) = (clamp_coord(x), clamp_coord(y));
                    list.jump_to(cx, cy, step_distance(last, cx, cy));
                    last = Some((cx, cy));
                }
                GalvoCommand::Mark { x, y } => {
                    let (cx, cy) = (clamp_coord(x), clamp_coord(y));
                    list.mark_to(cx, cy, step_distance(last, cx, cy));
                    last = Some((cx, cy));
                }
                GalvoCommand::SetPower { percent } => {
                    list.mark_current(power_ratio_from_percent(percent))
                }
                GalvoCommand::SetFrequency { hz } => {
                    list.qswitch_period(qswitch_period(f64::from(hz) / 1000.0))
                }
            }
        }
        list.finish()
    }

    /// Straight-line distance between two device points, clamped to a `u16`. The
    /// board reads this per-move distance to time the scanner through the move.
    fn step_distance(last: Option<(u16, u16)>, x: u16, y: u16) -> u16 {
        match last {
            None => 0,
            Some((lx, ly)) => {
                let dx = f64::from(x) - f64::from(lx);
                let dy = f64::from(y) - f64::from(ly);
                dx.hypot(dy).round().clamp(0.0, f64::from(u16::MAX)) as u16
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn a_packet_is_six_little_endian_u16() {
            // The end-of-list nop is 0x8002, little-endian 0x02 0x80.
            let p = encode(LIST_END_OF_LIST, [0; 5]);
            assert_eq!(p.len(), PACKET_LEN);
            assert_eq!(&p[0..2], &[0x02, 0x80]);
            assert!(p[2..].iter().all(|&b| b == 0));
        }

        #[test]
        fn a_jump_carries_its_coordinates() {
            let p = encode(LIST_JUMP_TO, [0x8000, 0x4000, 0, 0, 0]);
            assert_eq!(&p[0..2], &[0x01, 0x80]); // 0x8001
            assert_eq!(&p[2..4], &[0x00, 0x80]); // x = 0x8000
            assert_eq!(&p[4..6], &[0x00, 0x40]); // y = 0x4000
        }

        #[test]
        fn fiber_frequency_becomes_a_qswitch_period() {
            // 20000 / 20 kHz = 1000.
            assert_eq!(qswitch_period(20.0), 1000);
            assert_eq!(qswitch_period(0.0), 0);
        }

        #[test]
        fn a_negative_delay_sets_the_sign_slot() {
            // A Laser On TC of -300 us encodes as magnitude 300 with sign 0x8000.
            let params = delay_params(-300.0);
            assert_eq!(params[0], 300);
            assert_eq!(params[1], 0x8000);
            assert_eq!(delay_params(100.0)[1], 0x0000);
        }

        #[test]
        fn a_finished_list_is_whole_chunks() {
            let mut list = ListBuffer::new();
            list.jump_to(0x8000, 0x8000, 0);
            list.mark_to(0xA000, 0x8000, 0x2000);
            let bytes = list.finish();
            assert!(bytes.len().is_multiple_of(LIST_CHUNK_LEN));
            assert_eq!(bytes.len(), LIST_CHUNK_LEN);
        }

        #[test]
        fn compile_opens_with_the_mark_preamble() {
            let commands = [
                GalvoCommand::Jump {
                    x: 0x8000,
                    y: 0x8000,
                },
                GalvoCommand::Mark {
                    x: 0xC000,
                    y: 0x8000,
                },
            ];
            let bytes = compile(&commands, 1000, 100, 0x0800);
            // ready, dwell, write-port, jump speed, mark speed, fiber current.
            assert_eq!(&bytes[0..2], &LIST_READY_MARK.to_le_bytes());
            assert_eq!(&bytes[12..14], &LIST_DELAY.to_le_bytes());
            assert_eq!(&bytes[24..26], &LIST_WRITE_PORT.to_le_bytes());
            assert_eq!(&bytes[36..38], &LIST_JUMP_SPEED.to_le_bytes());
            assert_eq!(&bytes[38..40], &1000u16.to_le_bytes());
            assert_eq!(&bytes[48..50], &LIST_MARK_SPEED.to_le_bytes());
            assert_eq!(&bytes[50..52], &100u16.to_le_bytes());
            assert_eq!(&bytes[60..62], &LIST_MARK_CURRENT.to_le_bytes());
            assert_eq!(&bytes[62..64], &0x0800u16.to_le_bytes());
            // The first geometry command is a jump, after the 9-command preamble.
            assert_eq!(&bytes[108..110], &LIST_JUMP_TO.to_le_bytes());
            assert!(bytes.len().is_multiple_of(LIST_CHUNK_LEN));
        }
    }
}

#[cfg(test)]
mod cor_tests {
    use super::*;

    fn synthetic_cor() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(b"JCZ_COR_2_1\0");
        b.resize(32, 0);
        b.extend_from_slice(&1.0f32.to_le_bytes());
        b.resize(COR_HEADER_LEN, 0);
        // A 2x2 grid of corrections.
        for (dx, dy) in [(0i32, 0i32), (100, 0), (0, 100), (100, 100)] {
            b.extend_from_slice(&dx.to_le_bytes());
            b.extend_from_slice(&dy.to_le_bytes());
        }
        b
    }

    #[test]
    fn parses_and_samples_a_cor_grid() {
        let table = CorrectionTable::parse(&synthetic_cor());
        assert!(table.is_ok());
        if let Ok(table) = table {
            assert_eq!(table.size, 2);
            assert_eq!(table.scale, 1.0);
            // Corners read the grid cells directly.
            assert_eq!(table.correction_at(0.0, 0.0), (0.0, 0.0));
            assert_eq!(table.correction_at(1.0, 1.0), (100.0, 100.0));
            // The centre bilinearly averages to (50, 50).
            assert_eq!(table.correction_at(0.5, 0.5), (50.0, 50.0));
        }
    }

    #[test]
    fn rejects_a_non_cor_file() {
        assert!(CorrectionTable::parse(b"not a cor file at all......").is_err());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanlines_set_power_once_per_change() {
        let param = GalvoParam {
            field_size_mm: 100.0,
            resolution_bits: 16,
        };
        let runs = [
            MarkRun {
                y_mm: 0.0,
                x_start_mm: 0.0,
                x_end_mm: 10.0,
                power_percent: 60.0,
            },
            MarkRun {
                y_mm: 1.0,
                x_start_mm: 0.0,
                x_end_mm: 10.0,
                power_percent: 60.0,
            },
        ];
        let commands = mark_scanlines(&param, &runs, 30000);
        // Frequency, one power (shared), then jump+mark per run.
        let powers = commands
            .iter()
            .filter(|c| matches!(c, GalvoCommand::SetPower { .. }))
            .count();
        assert_eq!(powers, 1);
        let marks = commands
            .iter()
            .filter(|c| matches!(c, GalvoCommand::Mark { .. }))
            .count();
        assert_eq!(marks, 2);
    }

    #[test]
    fn field_center_maps_to_mid_scale() {
        let param = GalvoParam {
            field_size_mm: 100.0,
            resolution_bits: 16,
        };
        assert_eq!(param.device_max(), 65535);
        // Corner origin: 0 mm is device zero, the field centre (half) is mid scale.
        assert_eq!(param.logic_to_device(0.0, 0.0), (0, 0));
        let (x, y) = param.logic_to_device(50.0, 50.0);
        assert_eq!(x, 32768);
        assert_eq!(y, 32768);
    }

    #[test]
    fn mark_polyline_jumps_then_marks_and_closes() {
        let param = GalvoParam {
            field_size_mm: 100.0,
            resolution_bits: 16,
        };
        // A triangle centred near the field centre.
        let points = [(0.0, 0.0), (10.0, 0.0), (5.0, 8.0)];
        let commands = mark_polyline(&param, &points, true, 70.0, 20000);
        // Power, frequency, one jump, two marks along, one closing mark.
        assert_eq!(commands.len(), 6);
        assert_eq!(commands[0], GalvoCommand::SetPower { percent: 70.0 });
        assert_eq!(commands[1], GalvoCommand::SetFrequency { hz: 20000 });
        assert!(matches!(commands[2], GalvoCommand::Jump { .. }));
        assert!(matches!(commands[5], GalvoCommand::Mark { .. }));
        // The closing mark returns to the first point's device coordinate.
        let (fx, fy) = param.logic_to_device(0.0, 0.0);
        assert_eq!(commands[5], GalvoCommand::Mark { x: fx, y: fy });
    }

    #[test]
    fn field_edges_clamp_to_range() {
        let param = GalvoParam {
            field_size_mm: 100.0,
            resolution_bits: 16,
        };
        assert_eq!(param.logic_to_device(0.0, 100.0), (0, 65535));
        assert_eq!(param.logic_to_device(-999.0, 999.0), (0, 65535));
    }
}
