//! Robustness: a malformed EZCAD correction file must return an error, never
//! panic or abort. The loader reads untrusted `.cor` files from the machine
//! vendor, so every truncation and bad length has to fail cleanly.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use lasercraft_galvo::CorrectionTable;

fn with_header(data_len: usize) -> Vec<u8> {
    let mut v = b"JCZ_COR_2_1\0".to_vec();
    v.resize(40 + data_len, 0);
    v
}

#[test]
fn garbage_and_truncated_cor_never_panics() {
    let cases: Vec<Vec<u8>> = vec![
        vec![],
        b"JCZ_COR_2_1\0".to_vec(), // signature only, header too short
        vec![0u8; 40],             // header length, wrong signature
        with_header(0),            // header, no grid
        with_header(8),            // one point, a 1x1 grid is too small
        with_header(8 * 3),        // three points, not square
        with_header(7),            // data not a whole number of pairs
        with_header(8 * 4),        // a valid 2x2 grid
        vec![0xffu8; 200],         // arbitrary bytes
    ];
    for bytes in &cases {
        let _ = CorrectionTable::parse(bytes);
    }
}

#[test]
fn a_sweep_of_pseudo_random_bytes_never_panics() {
    let mut state: u64 = 0xd1b54a32d192ed03;
    for _ in 0..2000 {
        let len = (state % 300) as usize;
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            bytes.push((state >> 33) as u8);
        }
        let _ = CorrectionTable::parse(&bytes);
    }
}

#[test]
fn a_valid_two_by_two_grid_parses() {
    let table = CorrectionTable::parse(&with_header(8 * 4)).expect("a 2x2 grid should parse");
    assert_eq!(table.size, 2);
}
