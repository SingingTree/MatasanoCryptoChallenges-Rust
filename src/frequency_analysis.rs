use std::collections::btree_map::{BTreeMap, Entry};

// It would be nice if this could take an iter over u8s, but I couldn't figure out how to make that work
fn frequencies<'a, T: Ord>(collection : &'a [T]) -> BTreeMap<&'a T, usize> {
	let mut frequencies : BTreeMap<&T, usize> = BTreeMap::new();
	for item in collection.iter() {
		match frequencies.entry(item) {
			Entry::Vacant(entry) => { entry.insert(1); },
			Entry::Occupied(mut entry) => *entry.get_mut() += 1,
		}
	}
	return frequencies;
}

#[cfg(test)]
mod tests {
	use frequency_analysis;

	#[test]
	fn count_letters_test() {
		let hello: [char; 5] = ['h', 'e', 'l', 'l', 'o'];
		let freqs = frequency_analysis::frequencies(&hello);
		assert_eq!(freqs.get(&'l'), Some(&2));
	}
}
