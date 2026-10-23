//! Hexadecimal encoding for persisted SHA-256 digests.

use std::fmt::Write;

pub(crate) fn encode_hex(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(hex, "{byte:02x}").expect("writing to a String cannot fail");
    }
    hex
}
