use std::io::prelude::*;
use libintermix::client::pen::Pen;
use ::support::TestIO;
use term::terminfo::TermInfo;

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

fn assert_another_flush_does_nothing(pen: &mut Pen) {
    let mut io = TestIO::new();
    pen.flush(&mut io);

    let mut output: Vec<u8> = vec![];
    io.read_to_end(&mut output).unwrap();

    assert_eq!(output.len(), 0);
}

#[test]
fn new_pen_resets_cursor_to_origin() {
    let mut pen = Pen::new(TermInfo::from_name("xterm").unwrap());
    let mut io = TestIO::new();
    pen.flush(&mut io);

    let mut output: Vec<u8> = vec![];
    io.read_to_end(&mut output).unwrap();
    assert_has_bytes(&output, b"\x1b[1;1H");
}

#[test]
fn new_pen_resets_attributes_with_sgr0() {
    let mut pen = Pen::new(TermInfo::from_name("xterm").unwrap());
    let mut io = TestIO::new();
    pen.flush(&mut io);

    let mut output: Vec<u8> = vec![];
    io.read_to_end(&mut output).unwrap();
    assert_has_bytes(&output, b"\x1b[m");
}

#[test]
fn pen_is_silent_on_second_flush_without_changes() {
    let mut pen = Pen::new(TermInfo::from_name("xterm").unwrap());
    let mut io = TestIO::new();
    pen.flush(&mut io);

    let mut output: Vec<u8> = vec![];
    io.read_to_end(&mut output).unwrap();

    assert_another_flush_does_nothing(&mut pen);
}

#[test]
fn pen_can_turn_on_bold() {
    let mut pen = Pen::new(TermInfo::from_name("xterm").unwrap());
    let mut io = TestIO::new();

    pen.bold = true;
    pen.flush(&mut io);

    let mut output: Vec<u8> = vec![];
    io.read_to_end(&mut output).unwrap();
    assert_has_bytes(&output, b"\x1b[1m");

    assert_another_flush_does_nothing(&mut pen);
}

#[test]
fn pen_can_turn_off_bold() {
    let mut pen = Pen::new(TermInfo::from_name("xterm").unwrap());
    let mut io = TestIO::new();
    let mut output: Vec<u8> = vec![];

    pen.bold = true;
    pen.flush(&mut io);
    io.read_to_end(&mut output).unwrap();
    output.clear();

    pen.bold = false;
    pen.flush(&mut io);
    io.read_to_end(&mut output).unwrap();

    io.read_to_end(&mut output).unwrap();
    assert_has_bytes(&output, b"\x1b[m");

    assert_another_flush_does_nothing(&mut pen);
}
