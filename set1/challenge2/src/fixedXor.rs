extern crate "rustc-serialize" as rustc_serialize;
use rustc_serialize::hex::FromHex;

trait FixedXor {
	fn fixed_xor(&self, other: &Vec<u8>) -> Vec<u8>;
}

impl FixedXor for Vec<u8> {
	fn fixed_xor(self : &Vec<u8>, other : &Vec<u8>) {

	}
}

#[test]
fn test_xor() {
	let hex_buffer1 = "1c0111001f010100061a024b53535009181c".from_hex();
	let hex_buffer2 = "686974207468652062756c6c277320657965".from_hex();
	let expected_output = "746865206b696420646f6e277420706c6179".from_hex();
}