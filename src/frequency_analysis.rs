use std::collections::btree_map::{BTreeMap, Entry};
use std::iter::IntoIterator;


pub trait FrequencyAnalysable {
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

pub fn english_letter_frequencies() -> BTreeMap<char, f32> {
    let mut frequencies = BTreeMap::new();
    frequencies.insert('a', 0.08167);
    frequencies.insert('b', 0.01167);
    frequencies.insert('c', 0.02782);
    frequencies.insert('d', 0.04253);
    frequencies.insert('e', 0.12702);
    frequencies.insert('f', 0.02228);
    frequencies.insert('g', 0.02015);
    frequencies.insert('h', 0.06094);
    frequencies.insert('i', 0.06966);
    frequencies.insert('j', 0.00153);
    frequencies.insert('k', 0.00772);
    frequencies.insert('l', 0.04025);
    frequencies.insert('m', 0.02406);
    frequencies.insert('n', 0.06749);
    frequencies.insert('o', 0.07507);
    frequencies.insert('p', 0.01929);
    frequencies.insert('q', 0.00095);
    frequencies.insert('r', 0.05987);
    frequencies.insert('s', 0.06327);
    frequencies.insert('t', 0.09056);
    frequencies.insert('u', 0.02758);
    frequencies.insert('v', 0.00978);
    frequencies.insert('w', 0.02360);
    frequencies.insert('x', 0.00150);
    frequencies.insert('y', 0.01974);
    frequencies.insert('z', 0.00074);
    return frequencies;
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
