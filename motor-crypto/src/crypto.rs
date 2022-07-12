use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
use std::error::Error;
use std::fmt;

pub fn aes_encrypt_with_key(key: &[u8; 32], plain: Vec<u8>) -> Vec<u8> {
    let cipher = Aes256Gcm::new(key.into());
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    return cipher.encrypt(nonce, plain.as_ref()).unwrap();
}

pub fn aes_decrypt_with_key(key: &[u8; 32], ciphertext: Vec<u8>) -> Vec<u8> {
    let cipher = Aes256Gcm::new(key.into());
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    return cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
}
