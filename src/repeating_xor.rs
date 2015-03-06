use std::string::String;

trait RepeatingXorEncodable {
	fn repeating_xor_encode(self, rhs : Self) -> Self::Output;
	type Output;
}

impl<I : Iterator> RepeatingXorEncodable for I {}
	where <Self as Iterator>::Item : BitXor {

		repeating_xor_encode<u8>
	}
}



#[cfg(test)]
mod tests {

}