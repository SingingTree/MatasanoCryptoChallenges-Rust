use std::ops::BitXor;
use std::collections::btree_map::BTreeMap;

trait RepeatingXorEncodable {
	type Output;

	fn repeating_xor_encode(self, key : Self) -> Self::Output;	
}

impl<I : Iterator + Clone> RepeatingXorEncodable for I
where <Self as Iterator>::Item : BitXor {
	type Output = Result<Vec<<<Self as Iterator>::Item as BitXor>::Output>, &'static str>;

	fn repeating_xor_encode(self, key : Self) -> Result<Vec<<<Self as Iterator>::Item as BitXor>::Output>, &'static str>{
		let mut return_vec = Vec::new();
		let mut key_cycle = key.cycle();
		for item in self {
			match key_cycle.next() {
				Some(thing) => return_vec.push(item ^ thing),
				None => return Err("Error itering key for repeating XOR, none returned by iter where element expected")
			}
		}
		return Ok(return_vec);
	}
}

fn find_textual_decode_candidates(bytes : &[u8], character_frequencies : &BTreeMap<char, f32>) {
	for possible_key_len in range(1, 20) {
		// Break input bytes into chunks of possible_key_len, analyse those using single byte xor freq attack
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

		assert_eq!(plaintext_array.iter().repeating_xor_encode(key.iter()).unwrap(), expected_output);
	}

	#[test]
	fn test_string_repeating_xor() {
		let plaintext_string = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
		let key = "ICE";
		let expected_output = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".from_hex().unwrap();

		assert_eq!(plaintext_string.as_bytes().iter().repeating_xor_encode(key.as_bytes().iter()).unwrap(), expected_output);
	}
}