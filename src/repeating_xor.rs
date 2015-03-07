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

}