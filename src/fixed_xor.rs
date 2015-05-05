pub trait FixedXor<T> {
    fn fixed_xor(&self, other: & T) -> Result<Vec<u8>, String>;
}

impl FixedXor<Vec<u8>> for Vec<u8> {
    fn fixed_xor<'a>(self : &Vec<u8>, other: &'a Vec<u8>) -> Result<Vec<u8>, String> {
        if self.len() != other.len() {
            return Err("Input vectors do not have equal length".to_string());
        }

        let zip = self.iter().zip(other.iter());

        let result = zip.map(|(v1, v2) : (&u8, &u8)| v1 ^ v2).collect();

        return Ok(result);
    }
}

#[cfg(test)]
mod tests {
    use fixed_xor::FixedXor;
    use rustc_serialize::hex::FromHex;

    #[test]
    fn test_xor() {
        let hex_buffer1 = "1c0111001f010100061a024b53535009181c".from_hex().unwrap();
        let hex_buffer2 = "686974207468652062756c6c277320657965".from_hex().unwrap();
        let expected_output = "746865206b696420646f6e277420706c6179".from_hex().unwrap();

        let result_of_or : Vec<u8> = hex_buffer1.fixed_xor(&hex_buffer2).unwrap();;
        assert_eq!(expected_output, result_of_or);
    }
}