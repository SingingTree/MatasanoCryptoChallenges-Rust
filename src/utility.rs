use std::iter::IntoIterator;
use std::collections::btree_map::BTreeMap;
use std::cmp::Ordering;
use num::traits::Float;
use frequency_analysis;

pub trait ApproxEquality<T> {
    fn approx_equal(self, other: T) -> bool;
}

impl ApproxEquality<f32> for f32 {
    fn approx_equal(self, other : f32) -> bool {
        let epsilon : f32 = 0.000000001;
        return (self - other).abs() < epsilon;
    }
}

impl ApproxEquality<f64> for f64 {
    fn approx_equal(self, other : f64) -> bool {
        let epsilon : f64 = 0.000000001;
        return (self - other).abs() < epsilon;
    }
}

pub fn filter_strings_heuristically<II>(strings : II) -> Vec<String>
    where II : IntoIterator<Item = String> {
    let english_letter_freqs = frequency_analysis::english_letter_frequencies();
    let filtered_iter = strings.into_iter()
        // Cull strings that have a ratio of too many upper case chars
        .filter(|s| frequency_analysis::alphabetic_uppercase_frequency(s.chars()) < 0.5)
        // Cull strings that have a ratio of too many upper control chars
        .filter(|s| frequency_analysis::control_character_frequency(s.chars()) < 0.15);
    let mut output_strings : Vec<String> = filtered_iter.collect();
    sort_string_vec_by_char_freq(&mut output_strings, &english_letter_freqs);
    return output_strings;
}

#[inline]
pub fn sort_string_vec_by_char_freq(strings : &mut Vec<String>, character_frequencies : &BTreeMap<char, f32>) {
    strings.sort_by(|s1, s2| {
        let s1_distance = frequency_analysis::character_frequency_distance(s1.chars(), character_frequencies);
        let s2_distance = frequency_analysis::character_frequency_distance(s2.chars(), character_frequencies);
        if s1_distance < s2_distance {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}

#[cfg(test)]
mod tests {
    use utility::ApproxEquality;

    #[test]
    fn f32_approx_equal_test() {
        let num1 : f32 = 0.001;
        let num2 : f32 = 0.0010000000001;

        assert!(num1.approx_equal(num2));
        assert!(num2.approx_equal(num1));
    }

    #[test]
    fn f32_approx_not_equal_test() {
        let num1 : f32 = 0.001;
        let num2 : f32 = 0.00101;

        assert!(!num1.approx_equal(num2));
        assert!(!num2.approx_equal(num1));
    }
}