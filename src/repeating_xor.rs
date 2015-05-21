use std::ops::BitXor;
use std::collections::btree_map::BTreeMap;
use std::str::Chars;
use std::borrow::Borrow;
use single_byte_xor::SingleByteXorDecodable;
use utility;

trait RepeatingXorEncodable {
    type Output;

    fn repeating_xor_encode(self, key : Self) -> Self::Output;  
}

trait RepeatingXorDecodable {
    fn find_repeating_xor_textual_decode_candidates(&self, character_frequencies : &BTreeMap<char, f32>);
}

impl<I : Iterator + Clone> RepeatingXorEncodable for I
    where I::Item : BitXor {
    type Output = Result<Vec<<<Self as Iterator>::Item as BitXor>::Output>, &'static str>;

    fn repeating_xor_encode(self, key : Self) -> Result<Vec<<<Self as Iterator>::Item as BitXor>::Output>, &'static str>{
        let mut return_vec = Vec::new();
        let mut key_cycle = key.cycle();
        for item in self {
            match key_cycle.next() {
                Some(thing) => return_vec.push(item ^ thing),
                None => return Err("Error itering key for repeating XOR, none returned by iter where element expected")
            }
        }
        return Ok(return_vec);
    }
}

impl RepeatingXorDecodable for [u8] {
    fn find_repeating_xor_textual_decode_candidates(&self, character_frequencies : &BTreeMap<char, f32>) {
        if self.len() < 1 {
            return; // TODO: return an error
        }
        let mut min_edit_distance_and_key_len = (0, 0);

        for possible_key_len in 1..40 {
            if possible_key_len > self.len() / 2 {
                break;
            }
        }

        let mut bit_strings_to_decode : Vec<Vec<u8>> = Vec::new();
        for _i in 0..min_edit_distance_and_key_len.1 {
            bit_strings_to_decode.push(Vec::new());
        }

        for (i, byte) in self.iter().enumerate() {
            bit_strings_to_decode[i % min_edit_distance_and_key_len.1].push(*byte);
        }

        let mut decoded_strings : Vec<String> = Vec::new();
        for bit_string in &bit_strings_to_decode {
            let bit_string_borrow : &[u8] = bit_string.borrow();
            decoded_strings = utility::filter_strings_heuristically(decoded_strings);
            decoded_strings.push(bit_string_borrow.find_all_single_byte_xor_decode_candidates().remove(0));
        }
        
        let mut decoded_string_chars : Vec<Chars> = decoded_strings.iter().map(|x| x.chars()).collect();

        let mut decode_candidate_for_key_len = String::new();

        // for s in &decoded_strings {
        //  println!("{}", s);
        // }

        // for mut chars in decoded_string_chars {
        //  loop {
        //      match chars.next() {
        //          None => break,
        //          Some(c) => print!("{}", c)
        //      }
        //  }
                
        //  println!("");
        // }

        // for c in &decoded_string_chars[0] {
        //  println!("{}", c);
        // }

        let mut creating_decode_candidate = true;
        while creating_decode_candidate {
            for mut char_iter in decoded_string_chars.iter_mut() {
                match char_iter.next()  { // All broken here
                    None => creating_decode_candidate = false,
                    Some(c) => decode_candidate_for_key_len.push(c)
                }
            }
        }

        println!("{}", decode_candidate_for_key_len);
    }
}

#[cfg(test)]
mod tests {
    use repeating_xor::RepeatingXorEncodable;
    use rustc_serialize::hex::FromHex;

    #[test]
    fn test_array_u8_repeating_xor() {
        let plaintext_array = [0x00, 0xAA, 0xAA, 0x00];
        let key = [0x00, 0xAA];
        let mut expected_output = Vec::new(); 
        expected_output.push(0x00); expected_output.push(0x00); expected_output.push(0xAA); expected_output.push(0xAA);

        assert_eq!(plaintext_array.iter().repeating_xor_encode(key.iter()).unwrap(), expected_output);
    }

    #[test]
    fn test_string_repeating_xor() {
        let plaintext_string = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";
        let expected_output = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".from_hex().unwrap();

        assert_eq!(plaintext_string.as_bytes().iter().repeating_xor_encode(key.as_bytes().iter()).unwrap(), expected_output);
    }
}