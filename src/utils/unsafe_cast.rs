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

#[cfg(test)]
mod unsafe_cast_tests {
    use crate::crypto::aes_256_gcm::NonceSeq;
    use crate::utils::unsafe_cast::bytes_as_nonce;
    use crate::utils::unsafe_cast::tag_as_bytes;
    use ring::aead::Tag;

    #[test]
    pub fn tag_as_bytes_test() {
        let bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let tag = Tag::from(bytes);
        let tag_0 = Tag::from([0; 16]);
        let tag_1 = Tag::from([1; 16]);
        let tag_255 = Tag::from([255; 16]);
        assert_eq!(*tag_as_bytes(&tag), bytes);
        assert_eq!(*tag_as_bytes(&tag_0), [0u8; 16]);
        assert_eq!(*tag_as_bytes(&tag_1), [1u8; 16]);
        assert_eq!(*tag_as_bytes(&tag_255), [255u8; 16]);
    }

    #[test]
    pub fn bytes_as_nonce_test() {
        let min = u64::MIN;
        let one = 1;
        let max = u64::MAX;

        let nonce_min = NonceSeq(min);
        let nonce_one = NonceSeq(one);
        let nonce_max = NonceSeq(max);

        let byte_min: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let byte_one: [u8; 12] = [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0];
        let byte_max: [u8; 12] = [0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255];

        assert_eq!(nonce_min, bytes_as_nonce(byte_min));
        assert_eq!(nonce_one, bytes_as_nonce(byte_one));
        assert_eq!(nonce_max, bytes_as_nonce(byte_max));
    }
}
