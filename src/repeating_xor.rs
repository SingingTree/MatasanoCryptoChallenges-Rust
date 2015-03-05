trait RepeatingXorEncodable {
	fn repeating_xor_encode(self, xor_string : &String) -> Vec<u8>;
}

// Can't implement as below, because there aren't equality contraints on wheres yet :(
// impl<II : IntoIterator> RepeatingXorEncodable for II
// where <<Self as IntoIterator>::IntoIter as Iterator>::Item = u8 {
// }

// TODO: Can implement this using BitXor trait

impl<'a> RepeatingXorEncodable for &'a String {
	fn repeating_xor_encode(self, xor_string : &String) -> Vec<u8> {
		let mut xor_string_byte_cycle = xor_string.as_bytes().iter().cycle();
		let self_byte_iter = self.as_bytes().iter();

		let mut return_vec : Vec<u8> = Vec::new();

		for byte in self_byte_iter {
			return_vec.push(byte ^ xor_string_byte_cycle.next().unwrap());
		}

		return return_vec;
	}
}

#[cfg(test)]
mod tests {

}