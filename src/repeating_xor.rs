use std::ops::BitXor;
use std::collections::btree_map::BTreeMap;
use std::str::Chars;
use std::borrow::Borrow;
use single_byte_xor::SingleByteXorDecodable;
use utility;

pub trait RepeatingXorEncodable {
    type Output;

    fn repeating_xor_encode(self, key : Self) -> Self::Output;  
}

pub trait RepeatingXorDecodable {
    type Output;

    fn find_repeating_xor_decode(&self,
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
    type Output = Result<String, String>;

    fn find_repeating_xor_decode(&self,
                                 character_frequencies : &BTreeMap<char, f32>)
                                 -> Result<String, String> {

        println!("{:?}", self);
        // Find smallest edit distances, one of these is likely to be the key length
        if self.len() < 1 {
            return Ok(String::new());
        }
        let mut normalised_edit_distance_and_lengths =
            match utility::find_normalized_edit_distances(self) {
            Ok(x) => x,
            Err(e) => return Err(e)
        };
        // Got our edit distances, now we can use the top however many as possible keys

        // Create n vectors for each different char in the key
        // i.e. if the key is CATS, all chars encoded by the C should be in the first vec
        // A in the second vec, and so on and so forth
        match normalised_edit_distance_and_lengths.pop() {
            Some((_, len)) => {
                let supposed_key_length = len;
                let mut bit_strings_to_decode : Vec<Vec<u8>> = Vec::new();
                for _i in 0..supposed_key_length {
                    bit_strings_to_decode.push(Vec::new());
                }

                for (i, byte) in self.iter().enumerate() {
                    bit_strings_to_decode[i % supposed_key_length].push(*byte);
                }

                let mut decoded_strings : Vec<String> = Vec::new();
                for bit_string in &bit_strings_to_decode {
                    let bit_string_borrow : &[u8] = bit_string.borrow();
                    decoded_strings.push(
                        utility::filter_strings_heuristically(
                            bit_string_borrow.find_all_single_byte_xor_decodes()
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
                return Ok(decode_candidate);
            }
            None => return Err("Find repeating xor could not find any edit distances \
                                and couldn't calcuate any key sizes".to_owned())
        } 
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use repeating_xor::{RepeatingXorEncodable, RepeatingXorDecodable};
    use frequency_analysis::english_letter_frequencies;
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

    #[test]
    fn test_string_repeating_xor_round_trip() {
        let plaintext_string = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";
        let byte_vec = plaintext_string.as_bytes().iter().repeating_xor_encode(key.as_bytes().iter()).unwrap();
        let byte_slice : &[u8] = byte_vec.borrow();
        let decoded_string : String = byte_slice.find_repeating_xor_decode(&english_letter_frequencies()).unwrap();

        assert_eq!(decoded_string, plaintext_string);
    }
}