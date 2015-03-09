use std::ops::BitXor;

trait RepeatingXorEncodable {
	type Output;

	fn repeating_xor_encode(self, rhs : Self) -> Self::Output;	
}

impl<I : Iterator + Clone> RepeatingXorEncodable for I
where <Self as Iterator>::Item : BitXor {
	type Output = Vec<<<Self as Iterator>::Item as BitXor>::Output>;

	fn repeating_xor_encode(self, rhs : Self) -> Vec<<<Self as Iterator>::Item as BitXor>::Output>{
		let mut return_vec = Vec::new();
		let mut rhs_cycle = rhs.cycle();
		for item in self {
			return_vec.push(item ^ rhs_cycle.next().unwrap()); // TODO: use patterns for this
		}

		return return_vec;
	}
}



#[cfg(test)]
mod tests {
	use repeating_xor::RepeatingXorEncodable;
	use rustc_serialize::hex::FromHex;

	#[test]
	fn test_array_u8_repeating_xor() {
		let plaintext_array = [0x00, 0xAA, 0xAA, 0x00];
		let key = [0x00, 0xAA];
		let mut expected_output = Vec::new(); 
		expected_output.push(0x00); expected_output.push(0x00); expected_output.push(0xAA); expected_output.push(0xAA);

		assert_eq!(plaintext_array.iter().repeating_xor_encode(key.iter()), expected_output);
	}

	#[test]
	fn test_string_repeating_xor() {
		let plaintext_string = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
		let key = "ICE";
		let expected_output = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".from_hex().unwrap();

		assert_eq!(plaintext_string.as_bytes().iter().repeating_xor_encode(key.as_bytes().iter()), expected_output);
	}
}