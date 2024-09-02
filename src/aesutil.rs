use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use hex::{decode as hex_decode, encode as hex_encode};
use rand::{rngs::OsRng, RngCore};
use std::env;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub fn encrypt(plain_text: &str, key: &[u8; 16], iv: &[u8; 16]) -> Result<String, &'static str> {
    let data = plain_text.as_bytes();
    let cipher = Aes128Cbc::new_from_slices(key, iv).map_err(|_| "Invalid key or IV")?;
    let encrypted = cipher.encrypt_vec(data);
    Ok(hex_encode(encrypted))
}

pub fn decrypt(cipher_text: &str, key: &[u8; 16], iv: &[u8; 16]) -> Result<String, &'static str> {
    let decrypted = hex_decode(cipher_text).map_err(|_| "Hex decoding failed")?;
    let cipher = Aes128Cbc::new_from_slices(key, iv).map_err(|_| "Invalid key or IV")?;
    let decrypted = cipher
        .decrypt_vec(&decrypted)
        .map_err(|_| "Decryption failed")?;
    Ok(String::from_utf8_lossy(&decrypted).to_string())
}

pub fn generate_key_and_iv() -> ([u8; 16], [u8; 16]) {
    let seed = env::var("AES_SYS_KEY").unwrap_or_else(|_| "qazwsxedcrfv1234567890*#".to_string());
    let mut rng = OsRng {};
    let mut key = [0u8; 16];
    let mut iv = [0u8; 16];
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);
    (key, iv)
}
