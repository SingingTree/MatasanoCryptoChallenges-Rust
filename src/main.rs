extern crate rustc_serialize;
extern crate num;
extern crate rust_hamming_distance;

pub mod base64;
pub mod fixed_xor;
pub mod frequency_analysis;
pub mod single_byte_xor;
pub mod repeating_xor;
pub mod utility;

use repeating_xor::{RepeatingXorEncodable, RepeatingXorDecodable};
use std::borrow::Borrow;

fn main() {
    //let hex_bytes = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap();
    //repeating_xor::find_textual_decode_candidates(hex_bytes.borrow(), &frequency_analysis::english_letter_frequencies());

    let plaintext_string = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";
    let byte_vec = plaintext_string.as_bytes().iter().repeating_xor_encode(key.as_bytes().iter()).unwrap();
    let byte_slice : &[u8] = byte_vec.borrow();
    let decoded_string : String = byte_slice.find_repeating_xor_decode(&frequency_analysis::english_letter_frequencies()).unwrap();
    println!("{:?}", decoded_string);

    
}