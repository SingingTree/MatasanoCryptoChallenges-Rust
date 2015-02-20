use std::collections::btree_map::{BTreeMap, Entry};
use std::iter::IntoIterator;


trait FrequencyAnalysable {
	type Item : Ord;

	// Returns a BTreeMap from the items in self to their number of occurrences
	fn occurrences(self) -> BTreeMap<Self::Item, usize>;
	// Returns a BTreeMap from the items in self to their frequency%
	fn frequencies(self) -> BTreeMap<Self::Item, f32>;
}

fn occurrences_from_iter<I : Iterator>(iter : I) -> BTreeMap<<I as Iterator>::Item, usize>
where <I as Iterator>::Item: Ord {
    let mut occurrences = BTreeMap::new();
    for item in iter {
        match occurrences.entry(item) {
            Entry::Vacant(entry) => { entry.insert(1); }
            Entry::Occupied(entry) => { *entry.into_mut() += 1; }
        }
    }
    return occurrences;
}

fn frequencies_from_iter<I : Iterator>(iter : I) -> BTreeMap<<I as Iterator>::Item, f32>
where <I as Iterator>::Item: Ord {
    let mut occurrences = BTreeMap::new();
    let mut i = 0;
    for item in iter {
        match occurrences.entry(item) {
            Entry::Vacant(entry) => { entry.insert(1); }
            Entry::Occupied(entry) => { *entry.into_mut() += 1; }
        }
        i += 1;
    }
    let mut frequencies = BTreeMap::new();
    for (key, value) in occurrences.into_iter() {
        frequencies.insert(key, (value as f32) / (i as f32));
    }
    return frequencies;
}

impl<II : IntoIterator> FrequencyAnalysable for II
where <<Self as IntoIterator>::IntoIter as Iterator>::Item: Ord {
    type Item = <<Self as IntoIterator>::IntoIter as Iterator>::Item;

    fn occurrences(self) -> BTreeMap<<II as FrequencyAnalysable>::Item, usize> {
        return occurrences_from_iter(self.into_iter());
    }

    fn frequencies(self) -> BTreeMap<<II as FrequencyAnalysable>::Item, f32> {
    	return frequencies_from_iter(self.into_iter());
    }
}

impl<'a> FrequencyAnalysable for &'a str {
   type Item = char;

	fn occurrences(self) -> BTreeMap<char, usize> {
		return occurrences_from_iter(self.chars());
	}

	fn frequencies(self) -> BTreeMap<char, f32> {
		return frequencies_from_iter(self.chars());
	}
}


#[cfg(test)]
mod tests {
	use frequency_analysis::FrequencyAnalysable;

	#[test]
	fn string_occurrence_test() {
		let hello : &str = "Hello";
		let occurrences = hello.occurrences();
		assert_eq!(occurrences.get(&'l'), Some(&2));
	}

    #[test]
    fn string_frequency_test() {
        let hello : &str = "Hello";
        let freqs = hello.frequencies();
        assert!(freqs.get(&'o') > Some(&0.19) && freqs.get(&'o') < Some(&0.21));
    }

	#[test]
	fn num_slice_occurrence_test() {
		let nums = [5, 4, 3, 2, 1, 5];
		let occurrences = nums.occurrences();
		assert_eq!(occurrences.get(&5), Some(&2));
	}

    #[test]
    fn num_slice_frequency_test() {
        let nums = [5, 4, 3, 2, 5];
        let freqs = nums.frequencies();
        assert!(freqs.get(&5) > Some(&0.39) && freqs.get(&5) < Some(&0.41));
    }
}
