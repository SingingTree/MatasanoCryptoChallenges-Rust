use std::iter::IntoIterator;
use std::ops::BitXor;
use utility;
use frequency_analysis;

pub trait SingleByteXorDecodable {
    // Find all possible decode candidates for the input and return a vector containing them
    fn find_all_single_byte_xor_decodes(self) -> Vec<String>;
}

impl<'a, II> SingleByteXorDecodable for II 
    where II: IntoIterator<Item = &'a u8>, II::IntoIter : Clone {
    fn find_all_single_byte_xor_decodes(self) -> Vec<String> {
        //Brute force exploration of frequencies
        let mut possible_decodes : Vec<String> = Vec::new();
        let bytes_iter = self.into_iter();
        for i in 0..255 {
            let possible_decode : String = bytes_iter.clone().map(|b| b.bitxor(i) as char).collect();
            possible_decodes.push(possible_decode);
        }

        return possible_decodes;
    }
}

pub fn find_best_decodes_for_slice_heuristically(bit_strings : &[&[u8]]) -> Vec<String> {
    let mut best_decode_candidates : Vec<String> = Vec::new();
    for s in bit_strings {
        let mut bit_string_decodes = s.find_all_single_byte_xor_decodes();
        bit_string_decodes = utility::filter_strings_heuristically(bit_string_decodes);
        if bit_string_decodes.len() > 0 {
            best_decode_candidates.push(bit_string_decodes.remove(0));
        }
    }

    utility::sort_string_vec_by_char_freq(&mut best_decode_candidates, &frequency_analysis::english_letter_frequencies());

    return best_decode_candidates;
}

pub fn find_best_decodes_for_vec_heuristically(bit_strings : &Vec<Vec<u8>>) -> Vec<String> {
    let mut best_decode_candidates : Vec<String> = Vec::new();
    for s in bit_strings {
        let mut bit_string_decodes = s.find_all_single_byte_xor_decodes();
        bit_string_decodes = utility::filter_strings_heuristically(bit_string_decodes);
        if bit_string_decodes.len() > 0 {
            best_decode_candidates.push(bit_string_decodes.remove(0));
        }
    }

    utility::sort_string_vec_by_char_freq(&mut best_decode_candidates, &frequency_analysis::english_letter_frequencies());

    return best_decode_candidates;
}

#[cfg(test)]
mod tests {
    use rustc_serialize::hex::FromHex;
    use std::borrow::Borrow;
    use utility;
    use frequency_analysis::FrequencyAnalysable;
    use single_byte_xor::{SingleByteXorDecodable, find_best_decodes_for_slice_heuristically};

    #[test]
    fn frequencies_of_buffer() {
        let hex_buffer = "1c1c1c".from_hex().unwrap();
        let freqs = hex_buffer.frequencies();
        assert!(freqs.get(&0x1c) > Some(&0.99) && freqs.get(&5) < Some(&1.01));
    }

    #[test]
    fn matasano_find_single_byte_xor_plain_text() {
        let hex_bytes = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap();
        let hex_bytes_borrow : &[u8] = hex_bytes.borrow();
        let mut decode_candidates = hex_bytes_borrow.find_all_single_byte_xor_decodes();
        decode_candidates = utility::filter_strings_heuristically(decode_candidates);
        assert_eq!(decode_candidates.remove(0), "Cooking MC's like a pound of bacon");
    }

    #[test]
    fn find_single_byte_xor_in_list_of_candidates() {
        let text_bytes = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap(); // Encoded "Cooking MC's like a pound of bacon"
        let random_bytes_1 = "04050b447efd1efc28004ce63e85adb40c61d2cc3bf1d3a39c79f1091a3e96b810be".from_hex().unwrap();
        let random_bytes_2 = "f4930be3b09a0fd724ad4e1843e27494289c8b793e4bee12722fef52344aba8fe13e".from_hex().unwrap();
        let random_bytes_3 = "5cc78e04ab8ed6d9bce1d2d971b055e387b5b3414dc165e3f75b3cd957bc34a772d0".from_hex().unwrap();

        let encoded_slices : [&[u8]; 4] = [text_bytes.borrow(), random_bytes_1.borrow(), random_bytes_2.borrow(), random_bytes_3.borrow()];

        let mut decode_candidates = find_best_decodes_for_slice_heuristically(encoded_slices.borrow());

        assert_eq!(decode_candidates.remove(0), "Cooking MC's like a pound of bacon");
    }
}

