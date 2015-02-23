use std::cmp::Ordering;
use std::collections::btree_map::BTreeMap;
use frequency_analysis::{self, FrequencyAnalysable};

pub fn find_textual_decode_candidates(bytes : &[u8], character_frequencies : BTreeMap<char, f32>) {
	let byte_freqs = bytes.frequencies();
	let mut bytes_by_freq : Vec<(&u8, f32)> = byte_freqs.into_iter().collect();
	bytes_by_freq.sort_by(|&(_, a), &(_, b )| if a > b {Ordering::Less} else {Ordering::Greater});
	let mut characters_by_freq : Vec<(char, f32)> = character_frequencies.into_iter().collect();
	characters_by_freq.sort_by(|&(_, a), &(_, b )| if a > b {Ordering::Less} else {Ordering::Greater});

	for (k, v) in bytes_by_freq {
		println!("{}: {}", k, v);
	}

	for (k, v) in characters_by_freq {
		println!("{}: {}", k, v);
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