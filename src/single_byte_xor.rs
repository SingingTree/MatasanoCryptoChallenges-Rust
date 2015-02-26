use std::cmp::Ordering;
use std::collections::btree_map::BTreeMap;
use std::num::Float;
use frequency_analysis::{self, FrequencyAnalysable};

pub fn find_textual_decode_candidates(bytes : &[u8], character_frequencies : BTreeMap<char, f32>) {
	let byte_freqs = bytes.frequencies();
	let mut bytes_by_freq : Vec<(&u8, f32)> = byte_freqs.into_iter().collect();
	bytes_by_freq.sort_by(|&(_, a), &(_, b )| if a > b {Ordering::Less} else {Ordering::Greater});
	// let mut characters_by_freq : Vec<(char, f32)> = character_frequencies.into_iter().collect();
	// characters_by_freq.sort_by(|&(_, a), &(_, b )| if a > b {Ordering::Less} else {Ordering::Greater});

	//Brute force for exploration
	let mut possible_decodes : Vec<(Vec<char>, f32)> = Vec::new();
	for i in range(0, 255) {
		let possible_decode : Vec<char> = bytes.iter().map(|&b| (b ^ i) as char).collect();
		let possible_decode_freqs = possible_decode.clone().frequencies();
		let mut difference_from_specified_freqs : f32 = 0.0;
		for (k, v) in possible_decode_freqs {
			// Calculate freq difference here by taking difference between actual and ideal char occurrence and adding it to the difference
			match character_frequencies.get(&k.to_lowercase()) {
				None => {difference_from_specified_freqs += v}
            	Some(frequency) => { difference_from_specified_freqs += (*frequency - v).abs(); }
			}
		}
		possible_decodes.push((possible_decode, difference_from_specified_freqs));

		possible_decodes.sort_by(|&(_, f1), &(_, f2)| if f1 < f2 {Ordering::Less} else {Ordering::Greater});
	}

	for &(ref k, f) in possible_decodes.iter().take(10) {
		//let f : () = k;
		println!("{}: {:?}", f, k);
	}

}

#[cfg(test)]
mod tests {
	use rustc_serialize::hex::FromHex;
	use frequency_analysis::FrequencyAnalysable;

	#[test]
	fn frequencies_of_buffer() {
		let hex_buffer = "1c1c1c".from_hex().unwrap();
		let freqs = hex_buffer.frequencies();
		assert!(freqs.get(&0x1c) > Some(&0.99) && freqs.get(&5) < Some(&1.01));
	}
}