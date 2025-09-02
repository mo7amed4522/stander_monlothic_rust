//! Encryption and hashing utilities

use anyhow::Result;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}
pub fn verify_password(password: &str, hash: &str) -> bool {
    hash_password(password) == hash
}
pub fn generate_random_string(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
pub fn generate_token() -> String {
    generate_random_string(32)
}
pub fn encode_base64(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}
pub fn decode_base64(encoded: &str) -> Result<Vec<u8>> {
    general_purpose::STANDARD.decode(encoded)
        .map_err(|e| anyhow::anyhow!("Base64 decode error: {}", e))
}
pub fn xor_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .zip(key.iter().cycle())
        .map(|(d, k)| d ^ k)
        .collect()
}
pub fn xor_decrypt(encrypted: &[u8], key: &[u8]) -> Vec<u8> {
    xor_encrypt(encrypted, key)
}
pub fn generate_checksum(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}
pub fn verify_checksum(data: &[u8], expected_checksum: &str) -> bool {
    generate_checksum(data) == expected_checksum
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_password_hashing() {
        let password = "test_password";
        let hash = hash_password(password);
        assert!(verify_password(password, &hash));
        assert!(!verify_password("wrong_password", &hash));
    }
    #[test]
    fn test_random_string_generation() {
        let token1 = generate_random_string(16);
        let token2 = generate_random_string(16);
        assert_eq!(token1.len(), 16);
        assert_eq!(token2.len(), 16);
        assert_ne!(token1, token2);
    }
    #[test]
    fn test_base64_encoding() {
        let data = b"Hello, World!";
        let encoded = encode_base64(data);
        let decoded = decode_base64(&encoded).unwrap();
        assert_eq!(data, decoded.as_slice());
    }
    #[test]
    fn test_xor_encryption() {
        let data = b"Secret message";
        let key = b"key123";
        let encrypted = xor_encrypt(data, key);
        let decrypted = xor_decrypt(&encrypted, key);
        assert_eq!(data, decrypted.as_slice());
    }
    #[test]
    fn test_checksum() {
        let data = b"Important data";
        let checksum = generate_checksum(data);
        assert!(verify_checksum(data, &checksum));
        let modified_data = b"Modified data";
        assert!(!verify_checksum(modified_data, &checksum));
    }
}
