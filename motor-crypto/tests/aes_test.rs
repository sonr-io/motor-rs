#[cfg(test)]
mod tests {
    use aes_gcm::{Aes256Gcm, Key, Nonce};
    use motor_crypto::crypto::{aes_decrypt_with_key, aes_encrypt_with_key}; // Or `Aes128Gcm`

    // This test writes to a file
    #[test]
    fn test_encrypt_with_key() {
        let key = Key::from_slice(b"an example very very secret key.");

        let cipher = aes_encrypt_with_key(key.as_ref(), b"plaintext message".into());
        let plain = aes_decrypt_with_key(key.as_ref(), cipher.into());

        assert_eq!(plain, b"plaintext message")
    }
}
