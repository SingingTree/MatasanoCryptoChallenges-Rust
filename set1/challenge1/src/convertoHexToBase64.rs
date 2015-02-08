extern crate "rustc-serialize" as rustc_serialize;
use rustc_serialize::hex::FromHex;
use rustc_serialize::hex::ToHex;
use std::collections::HashMap;

// rustc-serialize could do all of this, but that would take some of the fun out of the challenge, I suppose

static BASE64_ENCODE_TABLE : [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F',
									'G', 'H', 'I', 'J', 'K', 'L',
									'M', 'N', 'O', 'P', 'Q', 'R',
									'S', 'T', 'U', 'V', 'W', 'X',
									'Y', 'Z', 'a', 'b', 'c', 'd',
									'e', 'f', 'g', 'h', 'i', 'j',
									'k', 'l', 'm', 'n', 'o', 'p',
									'q', 'r', 's', 't', 'u', 'v',
									'w', 'x', 'y', 'z', '0', '1',
									'2', '3', '4', '5', '6', '7',
									'8', '9', '+', '/'];

fn encode(bytes_to_encode : &Vec<u8>) -> Vec<char> {
	let num_bytes_to_encode : usize = bytes_to_encode.len();
	let mut encoded_bytes : Vec<char> = vec!();
	let mut i : usize = 0;

	// Handle all but the last bytes of the array
	while i < num_bytes_to_encode - 3 {
		let first_char_index : usize = ((bytes_to_encode[i] & 0xfc) >> 2) as usize;
		let second_char_index : usize = (((bytes_to_encode[i] & 0x03) << 4) | ((bytes_to_encode[i + 1] & 0xf0) >> 4)) as usize;
		let third_char_index : usize = (((bytes_to_encode[i + 1] & 0x0f) << 2) | (bytes_to_encode[i + 2] >> 6)) as usize;
		let fourth_char_index : usize = (bytes_to_encode[i + 2] & 0x3f) as usize;
		encoded_bytes.push(BASE64_ENCODE_TABLE[first_char_index]);
		encoded_bytes.push(BASE64_ENCODE_TABLE[second_char_index]);
		encoded_bytes.push(BASE64_ENCODE_TABLE[third_char_index]);
		encoded_bytes.push(BASE64_ENCODE_TABLE[fourth_char_index]);

		i += 3;
	}

	// Handle last bytes
	if num_bytes_to_encode - i == 3 {
		let first_char_index : usize = ((bytes_to_encode[i] & 0xfc) >> 2) as usize;
		let second_char_index : usize = (((bytes_to_encode[i] & 0x03) << 4) | ((bytes_to_encode[i + 1] & 0xf0) >> 4)) as usize;
		let third_char_index : usize = (((bytes_to_encode[i + 1] & 0x0f) << 2) | (bytes_to_encode[i + 2] >> 6)) as usize;
		let fourth_char_index : usize = (bytes_to_encode[i + 2] & 0x3f) as usize;
		encoded_bytes.push(BASE64_ENCODE_TABLE[first_char_index]);
		encoded_bytes.push(BASE64_ENCODE_TABLE[second_char_index]);
		encoded_bytes.push(BASE64_ENCODE_TABLE[third_char_index]);
		encoded_bytes.push(BASE64_ENCODE_TABLE[fourth_char_index]);

		i += 3;
	} else if num_bytes_to_encode - i == 2 {
		let first_char_index : usize = ((bytes_to_encode[i] & 0xfc) >> 2) as usize;
		let second_char_index : usize = (((bytes_to_encode[i] & 0x03) << 4) | ((bytes_to_encode[i + 1] & 0xf0) >> 4)) as usize;
		let third_char_index : usize = ((bytes_to_encode[i + 1] & 0x0f) << 2) as usize;
		encoded_bytes.push(BASE64_ENCODE_TABLE[first_char_index]);
		encoded_bytes.push(BASE64_ENCODE_TABLE[second_char_index]);
		encoded_bytes.push(BASE64_ENCODE_TABLE[third_char_index]);
		encoded_bytes.push('=');

		i += 2;
	} else if num_bytes_to_encode - i == 1 {
		let first_char_index : usize = ((bytes_to_encode[i] & 0xfc) >> 2) as usize;
		let second_char_index : usize = ((bytes_to_encode[i] & 0x03) << 4) as usize;
		encoded_bytes.push(BASE64_ENCODE_TABLE[first_char_index]);
		encoded_bytes.push(BASE64_ENCODE_TABLE[second_char_index]);
		encoded_bytes.push('=');
		encoded_bytes.push('=');	

		i += 1;
	}

	assert!(num_bytes_to_encode - i == 0, "There has been an error encoding to base64: <number of bytes to encode - bytes encoded> is non zero");

	return encoded_bytes;
}

fn decode(chars_to_decode : &Vec<char>) -> Result<Vec<u8>, String> {
	let num_bytes_to_decode = chars_to_decode.len();

	if num_bytes_to_decode % 4 != 0 {
		return Err("Incorrect number of base64 chars given to decode".to_string())
	}

	let mut decoded_bytes : Vec<u8> = vec!();

	let mut base64_decode_map : HashMap<char, u8> = HashMap::with_capacity(BASE64_ENCODE_TABLE.len()); 

	for (i, &c) in BASE64_ENCODE_TABLE.iter().enumerate() {
		base64_decode_map.insert(c, (i as u8));
	}

	let mut i : usize = 0;
	while i < num_bytes_to_decode - 4 {
		let first_value = base64_decode_map.get(&chars_to_decode[i]);
		let second_value = base64_decode_map.get(&chars_to_decode[i + 1]);
		let third_value = base64_decode_map.get(&chars_to_decode[i + 2]);
		let fourth_value = base64_decode_map.get(&chars_to_decode[i + 3]);
		let mut first_decoded_byte : u8 = 0x0;
		let mut second_decoded_byte : u8 = 0x0;
		let mut third_decoded_byte : u8 = 0x0;

		match first_value {
			Some(value) => first_decoded_byte |= (*value & 0x3f) << 2,
			None => return Err(format!("Unrecognised char in chars to decode: {}", &chars_to_decode[i]))
		}

		match second_value {
			Some(value) => {
				first_decoded_byte |= (*value & 0x30) >> 4;
				second_decoded_byte |= (*value & 0x0f) << 4;
			},
			None => return Err(format!("Unrecognised char in chars to decode: {}", &chars_to_decode[i + 1]))
		}

		match third_value {
			Some(value) => {
				second_decoded_byte |= (*value & 0x3c) >> 2;
				third_decoded_byte |= (*value & 0x03) << 6;
			},
			None => return Err(format!("Unrecognised char in chars to decode: {}", &chars_to_decode[i + 2]))
		}

		match fourth_value {
			Some(value) => third_decoded_byte |= value & 0x3f,
			None => return Err(format!("Unrecognised char in chars to decode: {}", &chars_to_decode[i + 3]))
		}

		decoded_bytes.push(first_decoded_byte);
		decoded_bytes.push(second_decoded_byte);
		decoded_bytes.push(third_decoded_byte);

		i += 4;
	}

	let first_value = base64_decode_map.get(&chars_to_decode[i]);
	let second_value = base64_decode_map.get(&chars_to_decode[i + 1]);
	let third_value = base64_decode_map.get(&chars_to_decode[i + 2]);
	let fourth_value = base64_decode_map.get(&chars_to_decode[i + 3]);
	let mut first_decoded_byte : u8 = 0x0;
	let mut second_decoded_byte : u8 = 0x0;
	let mut third_decoded_byte : u8 = 0x0;

	match first_value {
		Some(value) => first_decoded_byte |= (*value & 0x3f) << 2,
		None => return Err(format!("Unrecognised char in chars to decode: {}", &chars_to_decode[i]))
	}

	match second_value {
		Some(value) => {
			first_decoded_byte |= (*value & 0x30) >> 4;
			second_decoded_byte |= (*value & 0x0f) << 4;
		},
		None => return Err(format!("Unrecognised char in chars to decode: {}", &chars_to_decode[i + 1]))
	}

	match third_value {
		Some(value) => {
			second_decoded_byte |= (*value & 0x3c) >> 2;
			third_decoded_byte |= (*value & 0x03) << 6;
		},
		None => {
			if chars_to_decode[i + 2] != '=' {
				return Err(format!("Unrecognised char in chars to decode: {}", &chars_to_decode[i + 2]))
			}
		}
	}

	match fourth_value {
		Some(value) => third_decoded_byte |= value & 0x3f,
		None => {
			if chars_to_decode[i + 3] != '=' {
				return Err(format!("Unrecognised char in chars to decode: {}", &chars_to_decode[i + 3]))
			}
		}
	}

	decoded_bytes.push(first_decoded_byte);
	decoded_bytes.push(second_decoded_byte);
	if(chars_to_decode[i + 2] != '=') {
		decoded_bytes.push(third_decoded_byte);
	}



	println!("{:?}", num_bytes_to_decode - i);


	return Ok(decoded_bytes);
}

fn main() {
	// Testing stuff!
	let input_bytes1 = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".from_hex();
	
	match input_bytes1  {
		Ok(bytes) => {
			let encoded_bytes : Vec<char> = encode(&bytes);
			println!("Test: {:?}", encoded_bytes);
			let decoded_bytes = decode(&encoded_bytes);
			println!("Test {:?}",  decoded_bytes.unwrap().as_slice().to_hex());
		}
		Err(e) => {
			println!("Failed to convert input to bytes with error: {:?}", e);
		}

	}

}