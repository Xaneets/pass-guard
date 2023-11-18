extern crate ring;

use ring::digest;
use ring::digest::Digest;

pub fn sha256(input: &[u8]) -> Digest {
    let mut context = digest::Context::new(&digest::SHA256);
    context.update(input);
    context.finish()
}
