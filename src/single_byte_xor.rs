use std::iter::IntoIterator;
use std::ops::BitXor;
use utility;
use frequency_analysis;

pub trait SingleByteXorDecodable {
    // Find all possible decode candidates for the input and return a vector containing them
    fn find_all_single_byte_xor_decode_candidates(self) -> Vec<String>;
}

impl<'a, II> SingleByteXorDecodable for II 
    where II: IntoIterator<Item = &'a u8>, II::IntoIter : Clone {
    fn find_all_single_byte_xor_decode_candidates(self) -> Vec<String> {
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

pub fn find_best_decode_candidates_for_slice_heuristically(bit_strings : &[&[u8]]) -> Vec<String> {
    let mut best_decode_candidates : Vec<String> = Vec::new();
    for s in bit_strings {
        let mut bit_string_decodes = s.find_all_single_byte_xor_decode_candidates();
        bit_string_decodes = utility::filter_strings_heuristically(bit_string_decodes);
        if bit_string_decodes.len() > 0 {
            best_decode_candidates.push(bit_string_decodes.remove(0));
        }
    }

    utility::sort_string_vec_by_char_freq(&mut best_decode_candidates, &frequency_analysis::english_letter_frequencies());

    return best_decode_candidates;
}

pub fn find_best_decode_candidates_for_vec_heuristically(bit_strings : &Vec<Vec<u8>>) -> Vec<String> {
    let mut best_decode_candidates : Vec<String> = Vec::new();
    for s in bit_strings {
        let mut bit_string_decodes = s.find_all_single_byte_xor_decode_candidates();
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
    use single_byte_xor::{SingleByteXorDecodable, find_best_decode_candidates_for_slice_heuristically,
                          find_best_decode_candidates_for_vec_heuristically};

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
        let mut decode_candidates = hex_bytes_borrow.find_all_single_byte_xor_decode_candidates();
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

        let mut decode_candidates = find_best_decode_candidates_for_slice_heuristically(encoded_slices.borrow());

        assert_eq!(decode_candidates.remove(0), "Cooking MC's like a pound of bacon");

        let list_with_one_encoded_string : Vec<Vec<u8>> =
        "0e3647e8592d35514a081243582536ed3de6734059001e3f535ce6271032
        eb100fe63a031c4b35eb591845e428441c0d5b0037131f5c160a31243619
        22100330e03b4812e6120f163b1ef6abebe6f602545ef9a459e33d334c2a
        463405faa655563a43532cfe154bec32fe3345eb2c2700340811213e5006
        14241340112b2916017c270a0652732ee8121132385a6c020c040e2be15b
        251119225c573b105d5c0a371c3d421ef23e22377fee334e0228561b2d15
        2e4c2e373b434b0d0b1b340c300e4b195614130ea03c234c292e14530c46
        0d2c3f08560ee32e5a5b6413355215384442563e69ec294a0eef561e3053
        193c100c0b24231c012273e10d2e12552723586120020b02e45632265e5f
        2c175a11553d4b0b16025e2534180964245b125e5d6e595d1d2a0710580b
        213a175ff30855e4001b305000263f5a5c3c5100163cee00114e3518f33a
        10ed33e65b003012e7131e161d5e2e270b4645f358394118330f5a5b241b
        33e80130f45708395457573406422a3b0d03e6e5053d0d2d151c083337a2
        551be2082b1563c4ec2247140400124d4b6508041b5a472256093aea1847
        7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f
        0864eb4935144c501103a71851370719301bec57093a0929ea3f18060e55
        2d395e57143359e80efffb13330633ea19e323077b4814571e5a3de73a1f
        52e73c1d53330846243c422d3e1b374b5209543903e3195c041c251b7c04
        2f3c2c28273a12520b482f18340d565d1fe84735474f4a012e1a13502523
        23340f39064e306a08194d544647522e1443041d5ee81f5a18415e34a45f
        475a392637565757730a0c4a517b2821040e1709e028071558021f164c54
        100b2135190505264254005618f51152136125370eef27383e45350118ed
        3947452914e0223f1d040943313c193f295b221e573e1b5723391d090d1f
        2c33141859392b04155e3d4e393b322526ee3e581d1b3d6817374d0c085b
        c2ea5821200f1b755b2d13130f04e26625ea3a5b1e37144d3e473c24030d
        ee15025d2019f757305e3f010e2a453a205f1919391e1a04e86d1a350119
        1a5beb4946180fe0002a031a050b41e5164c58795021e1e45c59e2495c20
        1121394f1e381c3647005b7326250514272b55250a49183be5454ba518eb
        1ee55936102a465d5004371f2e382f1d03144f170d2b0eed042ee341eb19
        ec1014ef3ff1272c3408220a41163708140b2e340e505c560c1e4cf82704
        274b341a454a27a0263408292e362c201c0401462049523b2d55e5132d54
        e259032c444b091e2e4920023f1a7ce40908255228e36f0f2424394b3c48
        34130cf8223f23084813e745e006531a1e464b005e0e1ee405413fe22b4e
        4af201080c0928420c2d491f6e5121e451223b070dee54244b3efc470a0e
        771c161f795df81c22101408465ae7ef0c0604733ee03a20560c1512f217
        2f3a142c4155073a200f04166c565634020a59ea04244ff7413c4bc10858
        240d4752e5fa5a4e1ce255505602e55d4c575e2b59f52b4e0c0a0b464019
        21341927f3380232396707232ae424ea123f5b371d4f65e2471dfbede611
        4c071a57e9356ee415103c5c53e254063f2019340969e30a2e381d5b2555
        32042f46431d2c44607934ed180c1028136a5f2b26092e3b2c4e2930585a".split('\n').map(|x| x.from_hex().unwrap()).collect();

        decode_candidates = find_best_decode_candidates_for_vec_heuristically(&list_with_one_encoded_string);

        assert_eq!(decode_candidates.remove(0), "Now that the party is jumping\n");
    }
}

