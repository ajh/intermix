
// chars -> u32
// u32 -> char
// &[u8] -> vec<u8>
// vec<u8> -> &[u8]
// u8 -> char
// u8 -> u32
// &str -> String
// String -> &str

use std::char::from_u32;
use std::char;

#[test]
fn u8_to_char() {
    let a: u8 = b' '; // space
    let ch = char::from_u32(a as u32).unwrap();
    assert_eq!(ch, ' ');
}
