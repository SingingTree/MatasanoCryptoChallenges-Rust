extern crate "rustc-serialize" as rustc_serialize;

pub mod base64;
pub mod fixed_xor;
pub mod frequency_analysis;
pub mod single_byte_xor;
pub mod repeating_xor;
use rustc_serialize::hex::FromHex;

fn main() {
	let hex_bytes = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap();
	repeating_xor::find_textual_decode_candidates(hex_bytes.as_slice(), &frequency_analysis::english_letter_frequencies());
}