extern crate serialize;
use serialize::hex::FromHex;

static BASE64_TABLE : [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F',
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

	while(i < num_bytes_to_encode)
	{
		if num_bytes_to_encode - i >= 3 {
			let first_char_index : usize = (bytes_to_encode[i] >> 2) as usize;
			let second_char_index : usize = (((bytes_to_encode[i] & 0x03) << 4) | (bytes_to_encode[i + 1] >> 4)) as usize;
			let third_char_index : usize = (((bytes_to_encode[i + 1] & 0x0f) << 2) | (bytes_to_encode[i + 2] >> 6)) as usize;
			let fourth_char_index : usize = (bytes_to_encode[i + 2] & 0x3f) as usize;
			encoded_bytes.push(BASE64_TABLE[first_char_index]);
			encoded_bytes.push(BASE64_TABLE[second_char_index]);
			encoded_bytes.push(BASE64_TABLE[third_char_index]);
			encoded_bytes.push(BASE64_TABLE[fourth_char_index]);

			i += 3;
		} else if num_bytes_to_encode - i == 2 {
			let first_char_index : usize = (bytes_to_encode[i] >> 2) as usize;
			let second_char_index : usize = (((bytes_to_encode[i] & 0x03) << 4) | (bytes_to_encode[i + 1] >> 4)) as usize;
			let third_char_index : usize = ((bytes_to_encode[i + 1] & 0x0f) << 2) as usize;
			encoded_bytes.push(BASE64_TABLE[first_char_index]);
			encoded_bytes.push(BASE64_TABLE[second_char_index]);
			encoded_bytes.push(BASE64_TABLE[third_char_index]);
			encoded_bytes.push('=');

			i += 2;
		} else if num_bytes_to_encode - i == 1 {
			let first_char_index : usize = (bytes_to_encode[i] >> 2) as usize;
			let second_char_index : usize = ((bytes_to_encode[i] & 0x03) << 4) as usize;
			encoded_bytes.push(BASE64_TABLE[first_char_index]);
			encoded_bytes.push(BASE64_TABLE[second_char_index]);
			encoded_bytes.push('=');
			encoded_bytes.push('=');

			i += 1;
		}
	}

	return encoded_bytes;
}

fn main() {
	// Testing stuff!
	let input_bytes1 = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".from_hex();
	
	match input_bytes1 {
		Ok(bytes) => {
			let output_bytes : Vec<char> = encode(&bytes);
			println!("Test: {:?}", output_bytes);
		}
		Err(e) => {
			println!("Failed to convert input to bytes with error: {:?}", e);
		}

	}


	println!("Test");
}