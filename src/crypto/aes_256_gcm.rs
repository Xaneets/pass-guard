use ring::aead::{
    Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, Tag, UnboundKey, AES_256_GCM,
    NONCE_LEN,
};
use ring::error::Unspecified;

#[derive(Clone, Copy)]
pub struct NonceSeq(u64);

impl NonceSequence for NonceSeq {
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        let mut nonce_bytes = vec![0; NONCE_LEN];

        let bytes: [u8; 8] = self.0.to_be_bytes();
        nonce_bytes[4..].copy_from_slice(&bytes);

        Nonce::try_assume_unique_for_key(&nonce_bytes)
    }
}

#[allow(dead_code)]
pub struct Aes256Gcm;

impl Aes256Gcm {
    pub fn encrypt(
        mut data: Vec<u8>,
        hash: [u8; 32],
        nonce_value: NonceSeq,
    ) -> Result<(Vec<u8>, Tag, NonceSeq), Unspecified> {
        let unbound_key = UnboundKey::new(&AES_256_GCM, &hash)?;
        let mut sealing_key = SealingKey::new(unbound_key, nonce_value);
        let aad = Aad::empty();
        let tag = sealing_key.seal_in_place_separate_tag(aad, &mut data)?;

        Ok((data, tag, nonce_value))
    }

    pub fn decrypt(
        data: Vec<u8>,
        hash: [u8; 32],
        nonce_value: NonceSeq,
        tag: Tag,
    ) -> Result<(Vec<u8>, Tag, NonceSeq), Unspecified> {
        let unbound_key = UnboundKey::new(&AES_256_GCM, &hash)?;
        let aad = Aad::empty();

        let mut opening_key = OpeningKey::new(unbound_key, nonce_value);

        let mut data_with_tag = [&data, tag.as_ref()].concat();
        let decrypted_data = opening_key.open_in_place(aad, &mut data_with_tag)?;
        Ok((Vec::from(decrypted_data), tag, nonce_value))
    }
}
