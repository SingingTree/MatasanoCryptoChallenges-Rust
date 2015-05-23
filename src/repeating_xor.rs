use std::ops::BitXor;
use std::collections::btree_map::BTreeMap;
use std::str::Chars;
use std::borrow::Borrow;
use single_byte_xor::SingleByteXorDecodable;
use utility;
use rust_hamming_distance::bitwise_hamming_distance::BitwiseHammingDistancable;

trait RepeatingXorEncodable {
    type Output;

    fn repeating_xor_encode(self, key : Self) -> Self::Output;  
}

trait RepeatingXorDecodable {
    type Output;

    fn find_repeating_xor_textual_decode_candidates(&self,
                                                    character_frequencies : &BTreeMap<char, f32>)
                                                    -> Self::Output;
}

impl<I : Iterator + Clone> RepeatingXorEncodable for I
    where I::Item : BitXor {
    type Output = Result<Vec<<<Self as Iterator>::Item as BitXor>::Output>, &'static str>;

    fn repeating_xor_encode(self, key : Self)
                            -> Result<Vec<<<Self as Iterator>::Item as BitXor>::Output>, &'static str>{
        let mut return_vec = Vec::new();
        let mut key_cycle = key.cycle();
        for item in self {
            match key_cycle.next() {
                Some(thing) => return_vec.push(item ^ thing),
                None => return Err("Error itering key for repeating XOR, \
                    none returned by iter where element expected")
            }
        }
        return Ok(return_vec);
    }
}

impl RepeatingXorDecodable for [u8] {
    type Output = Result<Vec<String>, String>;

    fn find_repeating_xor_textual_decode_candidates(&self, 
                                                    character_frequencies : &BTreeMap<char, f32>)
                                                    -> Result<Vec<String>, String> {

        // Find smallest edit distance, this is likely to be the size of the key
        if self.len() < 1 {
            return Ok(Vec::new());
        }
        let mut min_edit_distance_and_key_len = (-1, 0);

        if self.len() <= 2 {
            min_edit_distance_and_key_len = (-1, 1);
        } else {
            let mut edit_distance = self[..1].bitwise_hamming_distance(&self[1..2]);
            match edit_distance {
                Ok(ed) => min_edit_distance_and_key_len = (ed, 1),
                Err(e) => return Err("find repeating xor failed attempting to calulate \
                                      hamming distance on iteration 1 with following \
                                      error\n".to_owned() + e)
            }
            for possible_key_len in 2..40 {
                if possible_key_len > self.len() / 2 {
                    break;
                }
                edit_distance = self[..possible_key_len].bitwise_hamming_distance(&self[possible_key_len..possible_key_len * 2]);
                 match edit_distance {
                    Ok(ed) => {
                        if ed < min_edit_distance_and_key_len.0 {
                            min_edit_distance_and_key_len = (ed, possible_key_len)
                        }
                    },
                    Err(e) => return Err("find repeating xor failed attempting to calulate \
                                      hamming distance on iteration ".to_owned() +
                                      &possible_key_len.to_string() +
                                      &" with following error\n".to_owned() + e)
                }
            }
        }
        // Got our edit distance, now use it to decode

        // Create n vectors for each different char in the key
        // i.e. if the key is CATS, all chars encoded by the C should be in the first vec
        // A in the second vec, and so on and so forth
        let suppose_key_length = min_edit_distance_and_key_len.1;
        let mut bit_strings_to_decode : Vec<Vec<u8>> = Vec::new();
        for _i in 0..suppose_key_length {
            bit_strings_to_decode.push(Vec::new());
        }

        for (i, byte) in self.iter().enumerate() {
            bit_strings_to_decode[i % suppose_key_length].push(*byte);
        }

        let mut decoded_strings : Vec<String> = Vec::new();
        for bit_string in &bit_strings_to_decode {
            let bit_string_borrow : &[u8] = bit_string.borrow();
            decoded_strings.push(
                utility::filter_strings_heuristically(
                    bit_string_borrow.find_all_single_byte_xor_decode_candidates()
                ).remove(0)
            );   
        }
        
        let mut decoded_string_chars : Vec<Chars> = decoded_strings.iter().map(|x| x.chars()).collect();

        let mut decode_candidate = String::new();

        let mut creating_decode_candidate = true;
        while creating_decode_candidate {
            for mut char_iter in decoded_string_chars.iter_mut() {
                match char_iter.next()  {
                    None => creating_decode_candidate = false,
                    Some(c) => decode_candidate.push(c)
                }
            }
        }

        println!("{}", decode_candidate);
        return Ok(Vec::new());
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