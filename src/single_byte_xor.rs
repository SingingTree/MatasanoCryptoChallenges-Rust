use std::cmp::Ordering;
use std::collections::btree_map::BTreeMap;
use std::iter::IntoIterator;
use std::ops::BitXor;
use frequency_analysis::{self, FrequencyAnalysable};
use utility::ApproxEquality;

pub trait SingleByteXorDecodable {
	// Find all possible decode candidates for the input and return a list vector of tuples - a decode candidate and the frequency of it occurring
	fn find_single_byte_xor_textual_decode_candidates(self, character_frequencies : &BTreeMap<char, f32>) -> Vec<(String, f32)>;
}

impl<'a, II : Copy + IntoIterator<Item = &'a u8>> SingleByteXorDecodable for II {
	fn find_single_byte_xor_textual_decode_candidates(self, character_frequencies : &BTreeMap<char, f32>) -> Vec<(String, f32)> {
		//Brute force exploration of frequencies
		let mut possible_decodes : Vec<(String, f32)> = Vec::new();
		for i in 0..255 {
			let possible_decode : String = self.into_iter().map(|b| b.bitxor(i) as char).collect();
			let possible_decode_freq_difference = frequency_analysis::character_frequency_distance(possible_decode.chars(), &character_frequencies);
			possible_decodes.push((possible_decode, possible_decode_freq_difference));
		}

		// Sort by frequency
		possible_decodes.sort_by(|&(_, f1), &(_, f2)| if f1 < f2 {Ordering::Less} else {Ordering::Greater});

		// Sort by special characters
		possible_decodes.sort_by(|&(ref d1, f1), &(ref d2, f2)|
		if f1.approx_equal(f2) {
			if frequency_analysis::control_character_frequency(d1.chars()) < frequency_analysis::control_character_frequency(d2.chars()) {
				Ordering::Less
			} else if frequency_analysis::control_character_frequency(d1.chars()) > frequency_analysis::control_character_frequency(d2.chars()) {
				Ordering::Greater
			} else {
				Ordering::Equal
			}
		} else {
			Ordering::Equal
		});

	// sort by number of uppercase letters (should probably filter on freq being close)
	// possible_decodes.sort_by(|&(ref d1, _), &(ref d2, _)|
	// 	if d1.to_ascii_lowercase() != d2.to_ascii_lowercase() {
	// 		Ordering::Equal
	// 	} else if frequency_analysis::alphabetic_uppercase_frequency(d1.chars()) < frequency_analysis::alphabetic_uppercase_frequency(d2.chars()) {
	// 		println!("!Equal");
	// 		Ordering::Less
	// 	} else {
	// 		println!("!Equal");
	// 		Ordering::Greater
	// 	});

	// for &(ref k, f) in possible_decodes.iter().take(10) {
	// 	//let f : () = k;
	// 	println!("{}: {:?}", f, k);
	// }

		return possible_decodes;
	}
}

pub fn find_best_decode_candidates_for_slice(bit_strings : &[&[u8]], character_frequencies : &BTreeMap<char, f32>) -> Vec<(String, f32)> {
	let mut best_decode_candidates : Vec<(String, f32)> = Vec::new();
	for s in bit_strings {
		best_decode_candidates.push(s.find_single_byte_xor_textual_decode_candidates(character_frequencies).remove(0));
	}

	// Sort candidates by frequency, the one with the least distance to our ideal char freqs will be at the start of the list
	best_decode_candidates.sort_by(|&(_, f1), &(_, f2)| if f1 < f2 {Ordering::Less} else {Ordering::Greater});

	return best_decode_candidates;
}

pub fn find_best_decode_candidates_for_vec(bit_strings : &Vec<Vec<u8>>, character_frequencies : &BTreeMap<char, f32>) -> Vec<(String, f32)> {
	let mut best_decode_candidates : Vec<(String, f32)> = Vec::new();
	for s in bit_strings {
		best_decode_candidates.push(s.find_single_byte_xor_textual_decode_candidates(character_frequencies).remove(0));
	}

	// Sort candidates by frequency, the one with the least distance to our ideal char freqs will be at the start of the list
	best_decode_candidates.sort_by(|&(_, f1), &(_, f2)| if f1 < f2 {Ordering::Less} else {Ordering::Greater});

	return best_decode_candidates;
}

#[cfg(test)]
mod tests {
	use rustc_serialize::hex::FromHex;
	use std::borrow::Borrow;
	use frequency_analysis::{self, FrequencyAnalysable};
	use single_byte_xor::{self, SingleByteXorDecodable, find_best_decode_candidates_for_slice};

	#[test]
	fn frequencies_of_buffer() {
		let hex_buffer = "1c1c1c".from_hex().unwrap();
		let freqs = hex_buffer.frequencies();
		assert!(freqs.get(&0x1c) > Some(&0.99) && freqs.get(&5) < Some(&1.01));
	}

	#[test]
	fn matasano_find_single_byte_xor_plain_text() {
		let hex_bytes = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap();
		let hex_bytes_borrow : &[u8] = hex_bytes.borrow();
		let mut decode_candidates = hex_bytes_borrow.find_single_byte_xor_textual_decode_candidates(&frequency_analysis::english_letter_frequencies());
		assert_eq!(decode_candidates.remove(0).0, "Cooking MC's like a pound of bacon");
	}

	#[test]
	fn find_single_byte_xor_in_list_of_candidates() {
		let text_bytes = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap(); // Encoded "Cooking MC's like a pound of bacon"
		let random_bytes_1 = "04050b447efd1efc28004ce63e85adb40c61d2cc3bf1d3a39c79f1091a3e96b810be".from_hex().unwrap();
		let random_bytes_2 = "f4930be3b09a0fd724ad4e1843e27494289c8b793e4bee12722fef52344aba8fe13e".from_hex().unwrap();
		let random_bytes_3 = "5cc78e04ab8ed6d9bce1d2d971b055e387b5b3414dc165e3f75b3cd957bc34a772d0".from_hex().unwrap();

		let mut encoded_slices : [&[u8]; 4] = [text_bytes.borrow(), random_bytes_1.borrow(), random_bytes_2.borrow(), random_bytes_3.borrow()];

		let mut decode_candidates = find_best_decode_candidates_for_slice(encoded_slices.borrow(), &frequency_analysis::english_letter_frequencies());

		assert_eq!(decode_candidates.remove(0).0, "Cooking MC's like a pound of bacon");
	}
}

