#[cfg(test)]
mod tests {
    use aes_gcm::{Aes256Gcm, Key, Nonce};
    use motor_crypto::crypto::aes_encrypt_with_key; // Or `Aes128Gcm`

    // This test writes to a file
    #[test]
    fn test_encrypt_with_key() {
        let key = Key::from_slice(b"an example very very secret key.");
        aes_encrypt_with_key(key.as_ref())
    }
}
