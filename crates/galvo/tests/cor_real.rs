//! Parse a real ComMarker B6 `jcz15.cor` correction file end to end.

use lasercraft_galvo::CorrectionTable;

#[test]
fn parses_the_real_commarker_cor() {
    let bytes = include_bytes!("jcz15.cor");
    let table = CorrectionTable::parse(bytes).expect("real .cor should parse");
    // The file is a 65x65 grid.
    assert_eq!(table.size, 65);
    // The field centre needs little correction.
    let (cx, cy) = table.correction_at(0.5, 0.5);
    assert!(cx.abs() < 10.0 && cy.abs() < 60.0, "centre {cx},{cy}");
    // A corner carries real lens correction.
    let (dx, dy) = table.correction_at(0.0, 0.0);
    assert!(dx.abs() > 100.0 || dy.abs() > 100.0, "corner {dx},{dy}");
}
