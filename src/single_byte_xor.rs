#[cfg(test)]
mod tests {
	use fixed_xor::FixedXor;
	use rustc_serialize::hex::FromHex;

	#[test]
	fn test_single_bytes() {
		let hex_buffer1 = "1c0111001f010100061a024b53535009181c".from_hex().unwrap();
		let hex_buffer2 = "686974207468652062756c6c277320657965".from_hex().unwrap();
		let expected_output = "746865206b696420646f6e277420706c6179".from_hex().unwrap();

		let result_of_or : Vec<u8> = hex_buffer1.fixed_xor(&hex_buffer2).unwrap();;
		assert_eq!(expected_output, result_of_or);
	}
}