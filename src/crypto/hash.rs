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
