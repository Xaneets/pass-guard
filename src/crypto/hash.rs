extern crate ring;

use hex;
use ring::digest;

pub fn sha256(input: &[u8]) -> [u8; 32] {
    let mut context = digest::Context::new(&digest::SHA256);
    context.update(input);
    let hash = context.finish();
    let result: [u8; 32] = hex::decode(format!("{:?}", hash).replace("SHA256:", ""))
        .expect("Failed to gen shapass")
        .as_slice()
        .try_into()
        .expect("Failed cast Vec<u8> to [u8; 32]");
    result
}

#[cfg(test)]
mod hash_tests {
    use crate::crypto::hash::sha256;
    use eframe::egui::util::hash;

    extern crate hex;

    #[test]
    pub fn sha256_test() {
        let input_1 = "".as_bytes();
        let input_2 = "1".as_bytes();
        let input_3 = "password".as_bytes();
        let input_4 = "1234567890!@#$%^&*)_+".as_bytes();

        let hash_1: [u8; 32] = [
            227, 176, 196, 66, 152, 252, 28, 20, 154, 251, 244, 200, 153, 111, 185, 36, 39, 174, 65, 228, 100, 155, 147, 76, 164, 149, 153,
            27, 120, 82, 184, 85,
        ];
        let hash_2: [u8; 32] = [
            107, 134, 178, 115, 255, 52, 252, 225, 157, 107, 128, 78, 255, 90, 63, 87, 71, 173, 164, 234, 162, 47, 29, 73, 192, 30, 82,
            221, 183, 135, 91, 75,
        ];
        let hash_3: [u8; 32] = [
            94, 136, 72, 152, 218, 40, 4, 113, 81, 208, 229, 111, 141, 198, 41, 39, 115, 96, 61, 13, 106, 171, 189, 214, 42, 17, 239, 114,
            29, 21, 66, 216,
        ];
        let hash_4: [u8; 32] = [
            108, 107, 124, 132, 237, 132, 152, 201, 174, 21, 223, 177, 81, 132, 249, 208, 128, 105, 116, 83, 65, 237, 39, 139, 164, 80, 26,
            4, 184, 249, 43, 8,
        ];

        assert_eq!(sha256(input_1), hash_1);
        assert_eq!(sha256(input_2), hash_2);
        assert_eq!(sha256(input_3), hash_3);
        assert_eq!(sha256(input_4), hash_4);
    }
}
