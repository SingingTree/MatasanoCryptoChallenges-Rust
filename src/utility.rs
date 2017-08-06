use std::iter::IntoIterator;
use std::collections::btree_map::BTreeMap;
use std::cmp::Ordering;
use frequency_analysis;
use rust_hamming_distance::bitwise_hamming_distance::BitwiseHammingDistancable;

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

pub fn filter_strings_heuristically<II>(strings : II) -> Vec<String> where
    II : IntoIterator<Item = String> {
    let filtered_iter = strings.into_iter()
        // Cull strings that have a ratio of too many upper case chars
        .filter(|s| frequency_analysis::alphabetic_uppercase_frequency(s.chars()) < 0.35)
        // Cull strings that have a ratio of too many control chars
        .filter(|s| frequency_analysis::control_character_frequency(s.chars()) < 0.10);
    let mut output_strings : Vec<String> = filtered_iter.collect();
    sort_string_vec_by_char_freq(&mut output_strings, &frequency_analysis::english_letter_frequencies());
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

pub fn find_normalized_edit_distances(bytes: &[u8]) -> Result<Vec<(f32, usize)>, String> {
    let mut normalised_edit_distance_and_lengths = Vec::new();

    if bytes.len() <= 2 {
        normalised_edit_distance_and_lengths.push((0f32, 1));
    } else {
        let mut edit_distance = bytes[..1].bitwise_hamming_distance(&bytes[1..2]);
        match edit_distance {
            Ok(ed) => normalised_edit_distance_and_lengths.push((ed as f32, 1)),
            Err(e) => return Err("Find repeating xor failed attempting to calulate \
                                hamming distance on iteration 1 with following \
                                error\n".to_owned() + e)
        }
        for possible_key_len in 2..40 {
            if possible_key_len > bytes.len() / 2 {
                break;
            }
            edit_distance = bytes[..possible_key_len].bitwise_hamming_distance(&bytes[possible_key_len..possible_key_len * 2]);
            match edit_distance {
                Ok(ed) => {
                    let normalised_ed = (ed as f32) / (possible_key_len as f32);
                    normalised_edit_distance_and_lengths.push((normalised_ed, possible_key_len))
                },
                Err(e) => return Err("Find repeating xor failed attempting to calulate \
                                    hamming distance on iteration ".to_owned() +
                                    &possible_key_len.to_string() +
                                    &" with following error\n".to_owned() + e)
            }
        }
    }
    normalised_edit_distance_and_lengths.sort_by(|&(ed1, _), &(ed2, _)|
        ed2.partial_cmp(&ed1).unwrap());
    for &(ed, len) in normalised_edit_distance_and_lengths.iter() {
        println!("{:?}, {:?}", ed, len);
    }
    Ok(normalised_edit_distance_and_lengths)
}

#[cfg(test)]
mod tests {
    use utility::ApproxEquality;

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
}