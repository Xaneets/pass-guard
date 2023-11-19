use crate::crypto::aes_256_gcm::NonceSeq;
use ring::aead::{Tag, NONCE_LEN};

pub fn tag_as_bytes(tag: &Tag) -> &[u8; 16] {
    unsafe { std::mem::transmute::<&Tag, &[u8; 16]>(tag) }
}

pub fn bytes_as_tag(bytes: [u8; 16]) -> &'static Tag {
    unsafe { std::mem::transmute::<&[u8; 16], &Tag>(&bytes) }
}

pub fn bytes_as_nonce(bytes: [u8; 12]) -> NonceSeq {
    let mut last_8_bytes: [u8; 8] = [0; 8];
    last_8_bytes.copy_from_slice(&bytes[4..]);
    let val = unsafe { std::mem::transmute::<[u8; NONCE_LEN - 4], u64>(last_8_bytes) };
    NonceSeq(val)
}
