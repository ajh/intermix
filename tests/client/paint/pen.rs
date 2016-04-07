use std::io::prelude::*;
use libintermix::client::paint::Pen;
use term::terminfo::TermInfo;
use term::terminfo::parm::Variables;

fn find_subsequence_slowly(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn assert_has_bytes(actual: &[u8], expected: &[u8]) {
    if find_subsequence_slowly(actual, expected).is_none() {
        let a = stringify_byte_array_as_hex(actual);
        let e = stringify_byte_array_as_hex(expected);
        assert!(false, format!("expected {:?} to contain {:?}", a, e));
    }
}

fn stringify_byte_array_as_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:x} ", b)).collect()
}

fn flush_pen(pen: &mut Pen) -> Vec<u8> {
    let terminfo = TermInfo::from_name("xterm").unwrap();
    let mut vars = Variables::new();
    pen.flush(&terminfo, &mut vars)
}

#[test]
fn new_pen_resets_cursor_to_origin() {
    let mut pen = Pen::new();
    assert_has_bytes(&flush_pen(&mut pen), b"\x1b[1;1H");
}

#[test]
fn new_pen_resets_attributes_with_sgr0() {
    let mut pen = Pen::new();
    assert_has_bytes(&flush_pen(&mut pen), b"\x1b[m");
}

#[test]
fn pen_is_silent_on_second_flush_without_changes() {
    let mut pen = Pen::new();
    flush_pen(&mut pen);
    assert_eq!(flush_pen(&mut pen).len(), 0);
}

#[test]
fn pen_can_turn_on_bold() {
    let mut pen = Pen::new();

    pen.bold = true;
    assert_has_bytes(&flush_pen(&mut pen), b"\x1b[1m");
    assert_eq!(flush_pen(&mut pen).len(), 0);
}

#[test]
fn pen_can_turn_off_bold() {
    let mut pen = Pen::new();

    pen.bold = true;
    flush_pen(&mut pen);

    pen.bold = false;
    assert_has_bytes(&flush_pen(&mut pen), b"\x1b[m");
    assert_eq!(flush_pen(&mut pen).len(), 0);
}
