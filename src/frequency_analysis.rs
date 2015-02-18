use std::collections::btree_map::{BTreeMap, Entry};


trait FrenquencyAnalysable {
	type Item : Ord;

	fn frequencies(self) -> BTreeMap<Self::Item, usize>;
}


impl<'a, T : Ord> FrenquencyAnalysable for &'a [T] {
	type Item = &'a T;

	fn frequencies(self) -> BTreeMap<&'a T, usize> {
		let mut frequencies : BTreeMap<&'a T, usize> = BTreeMap::new();
		for item in self.iter() {
			match frequencies.entry(item) {
				Entry::Vacant(entry) => { entry.insert(1); },
				Entry::Occupied(mut entry) => *entry.get_mut() += 1,
			}
		}
		return frequencies;
	}
}

impl<'a> FrenquencyAnalysable for &'a str {
	type Item = char;

	fn frequencies(self) -> BTreeMap<char, usize> {
		let mut frequencies : BTreeMap<char, usize> = BTreeMap::new();
		for item in self.chars() {
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
		let hello : &str = "Hello";
		let freqs = hello.frequencies();
		assert_eq!(freqs.get(&'l'), Some(&2));
	}

	#[test]
	fn count_nums_test() {
		let mut nums = [5, 4, 3, 2, 1, 5];
		let freqs = nums.frequencies();
		assert_eq!(freqs.get(&5), Some(&2));
	}
}
