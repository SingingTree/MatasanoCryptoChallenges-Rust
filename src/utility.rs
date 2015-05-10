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
    let filtered_iter = strings.into_iter()
        // Cull strings that have a ratio of too many upper case chars
        .filter(|s| frequency_analysis::alphabetic_uppercase_frequency(s.chars()) < 0.5)
        // Cull strings that have a ratio of too many upper control chars
        .filter(|s| frequency_analysis::control_character_frequency(s.chars()) < 0.15);
    let mut output_strings : Vec<String> = filtered_iter.collect();
    sort_string_vec_by_char_freq(&mut output_strings, &frequency_analysis::english_letter_frequencies());
    return output_strings;
}

pub trait HammingDistancable<T> {
    type Output;
    fn hamming_distance(self, other: T) -> Self::Output;
}

impl<'a> HammingDistancable<&'a u8> for &'a u8 {
    type Output = u32;
    fn hamming_distance(self, other: &u8) -> u32 {
        let mut distance : u32 = 0;
        if (self ^ other) & 0x01 > 0 { distance += 1}
        if ((self ^ other) >> 1) & 0x01 > 0 { distance += 1}
        if ((self ^ other) >> 2) & 0x01 > 0 { distance += 1}
        if ((self ^ other) >> 3) & 0x01 > 0 { distance += 1}
        if ((self ^ other) >> 4) & 0x01 > 0 { distance += 1}
        if ((self ^ other) >> 5) & 0x01 > 0 { distance += 1}
        if ((self ^ other) >> 6) & 0x01 > 0 { distance += 1}
        if ((self ^ other) >> 7) & 0x01 > 0 { distance += 1}
        return distance;
    }
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
    use utility::HammingDistancable;

    #[test]
    fn f32_approx_equal() {
        let num1 : f32 = 0.001;
        let num2 : f32 = 0.0010000000001;

        assert!(num1.approx_equal(num2));
        assert!(num2.approx_equal(num1));
    }

    #[test]
    fn f32_approx_not_equal() {
        let num1 : f32 = 0.001;
        let num2 : f32 = 0.00101;

        assert!(!num1.approx_equal(num2));
        assert!(!num2.approx_equal(num1));
    }

    #[test]
    fn u8_hamming_distance() {
        let byte1 : u8 = 0x01;
        let byte2 : u8 = 0x03;

        assert!(byte1.hamming_distance(&byte2) == 1);
        assert!(byte2.hamming_distance(&byte1) == 1);

        let byte3 : u8 = 0x01;
        let byte4 : u8 = 0xFF;

        assert!(byte3.hamming_distance(&byte4) == 7);
        assert!(byte4.hamming_distance(&byte3) == 7);
    }
}