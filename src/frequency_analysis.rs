use std::collections::btree_map::{BTreeMap, Entry};


trait FrenquencyAnalysable {
	type Item : Ord;

	fn frequencies(&self) -> BTreeMap<&Self::Item, usize>;
}


impl<T : Ord> FrenquencyAnalysable for [T] {
	type Item = T;

	fn frequencies<'a>(self : &'a [T]) -> BTreeMap<&'a T, usize> {
		let mut frequencies : BTreeMap<&T, usize> = BTreeMap::new();
		for item in self.iter() {
			match frequencies.entry(item) {
				Entry::Vacant(entry) => { entry.insert(1); },
				Entry::Occupied(mut entry) => *entry.get_mut() += 1,
			}
		}
		return frequencies;
	}
}

#[cfg(test)]
mod tests {
	use frequency_analysis::FrenquencyAnalysable;

	#[test]
	fn count_letters_test() {
		let hello: [char; 5] = ['h', 'e', 'l', 'l', 'o'];
		let freqs = hello.frequencies();
		assert_eq!(freqs.get(&'l'), Some(&2));
	}
}
