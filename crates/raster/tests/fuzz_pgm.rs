//! Robustness: a malformed PGM must return an error, never panic or abort.
//!
//! The PGM loader reads untrusted files, so every header shape has to fail
//! cleanly. Under wasm a panic aborts the module, so this is a security test.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use lasercraft_raster::GrayImage;

#[test]
fn garbage_and_truncated_pgm_never_panics() {
    let cases: &[&[u8]] = &[
        b"",
        b"P5",
        b"P5 ",
        b"P6 1 1 255 x",                  // wrong magic
        b"P5 0 0 255 ",                   // zero dimensions
        b"P5 4294967295 4294967295 255 ", // dimensions that overflow a product
        b"P5 2 2 0 ",                     // zero maxval
        b"P5 2 2 300 ",                   // maxval over 255
        b"P5 2 2 255 ",                   // header fine, no pixel data
        b"P5 2 2 255 \x00",               // one pixel short
        &[0xff; 64],                      // arbitrary bytes
    ];
    for bytes in cases {
        // The only contract is that this returns, Ok or Err, without unwinding.
        let _ = GrayImage::from_pgm(bytes);
    }
}

#[test]
fn a_sweep_of_pseudo_random_bytes_never_panics() {
    // A small deterministic generator keeps the test reproducible without a dep.
    let mut state: u64 = 0x9e3779b97f4a7c15;
    for _ in 0..2000 {
        let len = (state % 96) as usize;
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            bytes.push((state >> 33) as u8);
        }
        let _ = GrayImage::from_pgm(&bytes);
    }
}

#[test]
fn a_valid_tiny_pgm_parses() {
    let img = GrayImage::from_pgm(b"P5 2 1 255 \x00\xff").expect("valid PGM should parse");
    assert_eq!(img.width, 2);
    assert_eq!(img.height, 1);
    assert_eq!(img.pixels, vec![0x00, 0xff]);
}
