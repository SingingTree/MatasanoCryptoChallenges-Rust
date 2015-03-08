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

	#[test]
	fn test_array_u8_repeating_xor() {
		let plaintext_vec = [0x00, 0xAA, 0xAA, 0x00];
		let key = [0x00, 0xAA];
		let mut expected_output = Vec::new(); 
		expected_output.push(0x00); expected_output.push(0x00); expected_output.push(0xAA); expected_output.push(0xAA);

		assert_eq!(plaintext_vec.iter().repeating_xor_encode(key.iter()), expected_output);
	}
}