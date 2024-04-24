use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};

pub fn process_chacha20_encrypt(key: &[u8; 32], input: &[u8]) -> Vec<u8> {
    let cipher = ChaCha20Poly1305::new_from_slice(key).unwrap();
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, input).unwrap();
    let mut res = Vec::new();
    res.extend_from_slice(&nonce);
    res.extend_from_slice(&ciphertext);
    res
}

pub fn process_chacha20_decrypt(key: &[u8; 32], input: &[u8]) -> Vec<u8> {
    let cipher = ChaCha20Poly1305::new_from_slice(key).unwrap();
    let nonce = Nonce::from_slice(&input[0..12]);
    let ciphertext = &input[12..];
    let plaintext = cipher.decrypt(nonce, ciphertext).unwrap();
    plaintext.to_vec()
}

pub fn gen_chacha20_key(password: &str) -> [u8; 32] {
    let mut key = [0u8; 32];
    let salt: &[u8] = b"salt"; // 写死的盐值吗，这样安全吗？
    pbkdf2::pbkdf2_hmac::<sha2::Sha256>(password.as_bytes(), salt, 100, &mut key);
    key
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_pbkdf2() {
        let mut key1 = [0u8; 32];
        pbkdf2::pbkdf2_hmac::<sha2::Sha256>(b"password", b"salt", 100, &mut key1);
        println!("{:?}", key1);
    }

    #[test]
    fn test_gen_chacha20_encrypt_decrypt() {
        let key = super::gen_chacha20_key("password");
        let plaintext = b"plaintext message";
        let ciphertext = super::process_chacha20_encrypt(&key, plaintext);
        let decrypt_res = super::process_chacha20_decrypt(&key, &ciphertext);
        assert_eq!(&decrypt_res, plaintext);
    }
}
