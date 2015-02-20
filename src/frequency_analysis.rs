use std::collections::btree_map::{BTreeMap, Entry};
use std::iter::IntoIterator;


trait FrequencyAnalysable {
	type Item : Ord;

	fn occurrences(self) -> BTreeMap<Self::Item, usize>;
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

impl<II : IntoIterator> FrequencyAnalysable for II
where <<Self as IntoIterator>::IntoIter as Iterator>::Item: Ord {
    type Item = <<Self as IntoIterator>::IntoIter as Iterator>::Item;

    fn occurrences(self) -> BTreeMap<<II as FrequencyAnalysable>::Item, usize> {
        return occurrences_from_iter(self.into_iter());
    }
}

impl<'a> FrequencyAnalysable for &'a str {
   type Item = char;

	fn occurrences(self) -> BTreeMap<char, usize> {
		return occurrences_from_iter(self.chars());
	}
}


#[cfg(test)]
mod tests {
	use frequency_analysis::FrequencyAnalysable;

	#[test]
	fn count_letters_test() {
		let hello : &str = "Hello";
		let freqs = hello.occurrences();
		assert_eq!(freqs.get(&'l'), Some(&2));
	}

	#[test]
	fn count_nums_test() {
		let nums = [5, 4, 3, 2, 1, 5];
		let freqs = nums.occurrences();
		assert_eq!(freqs.get(&5), Some(&2));
	}
}
