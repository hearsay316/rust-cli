use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use encoding_rs::Encoding;
use rand::{random, Rng};
use std::env;

// Type alias for AES128-CBC with PKCS7 padding
//带PKCS7填充的AES128-CBC的类型别名
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn generate_key(key_seed: Option<&str>) -> Vec<u8> {
    let default_seed = "qazwsxedcrfv1234567890*#".to_string();

    // 从环境变量AES_SYS_KEY读取
    let key_seed = key_seed
        .map(|s| s.to_string())
        .or_else(|| env::var("AES_SYS_KEY").ok()) // 尝试读取环境变量
        .unwrap_or(default_seed);

    let seed_bytes = key_seed.as_bytes();

    // 确保AES密钥长度为16字节
    let mut key = vec![0u8; 16];
    let mut rng = rand::thread_rng();
    rng.fill(&mut key[..]);
    key[..seed_bytes.len().min(16)].copy_from_slice(&seed_bytes[..seed_bytes.len().min(16)]);
    key
}

// 将二进制转换为16进制字符串
fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex_str = String::new();
    for byte in bytes {
        hex_str.push_str(&format!("{:02X}", byte));
    }
    hex_str
}

// 将16进制字符串转换为二进制
fn hex_to_bytes(hex_str: &str) -> Vec<u8> {
    (0..hex_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16).unwrap())
        .collect()
}

// Encrypt the given text with AES-CBC
pub fn encrypt(plain_text: &str, key_seed: Option<&str>, charset: &str) -> Option<String> {
    let key = generate_key(key_seed);
    let iv = random::<[u8; 16]>(); // 16-byte IV for AES128-CBC

    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();

    // 使用指定字符集将明文转换为字节
    let (text_bytes, _, _) = Encoding::for_label(charset.as_bytes())
        .unwrap_or(encoding_rs::UTF_8)
        .encode(plain_text);

    let cipher_text = cipher.encrypt_vec(&text_bytes);

    // 将密文与IV一起编码为16进制字符串，并返回
    let result = format!("{}{}", bytes_to_hex(&iv), bytes_to_hex(&cipher_text));
    Some(result)
}

// Decrypt the given cipher text (in hex format)
pub fn decrypt(cipher_text_hex: &str, key_seed: Option<&str>, charset: &str) -> Option<String> {
    let key = generate_key(key_seed);

    // 将输入的16进制字符串解码为字节
    let cipher_bytes = hex_to_bytes(cipher_text_hex);

    // 提取前16字节作为IV，剩余的作为密文
    let (iv, cipher_text) = cipher_bytes.split_at(16); // AES128 uses 16-byte IV
    let cipher = Aes128Cbc::new_from_slices(&key, iv).unwrap();

    match cipher.decrypt_vec(cipher_text) {
        Ok(decrypted_bytes) => {
            // 使用指定的字符集将字节转换为字符串
            let (decoded_text, _, _) = Encoding::for_label(charset.as_bytes())
                .unwrap_or(encoding_rs::UTF_8)
                .decode(&decrypted_bytes);
            Some(decoded_text.into_owned())
        }
        Err(_) => None,
    }
}

// fn main() {
//     // 加密示例
//     let encrypted = encrypt("support@dealsdayone.com", None, "utf-8");
//     println!("Encrypted: {:?}", encrypted);
//
//     // 解密示例
//     if let Some(encrypted_text) = encrypted {
//         let decrypted = decrypt(&encrypted_text, None, "utf-8");
//         println!("Decrypted: {:?}", decrypted);
//     }
// }
