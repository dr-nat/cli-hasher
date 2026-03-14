use sha_256::Sha256;
use std::error::Error;
use crate::get_input;

pub fn hash() -> Result<[u8; 32], Box<dyn Error>> {
    let mut sha256: Sha256 = Sha256::new();

    let user_input = get_input::get_args()?;
    let input_bytes: &[u8] = user_input.as_bytes();

    let hash: [u8; 32] = sha256.digest(input_bytes);

    Ok(hash)
}

pub fn hash_hex(hash: &[u8]) -> String {
    let mut hex = String::new();

    for bytes in hash{
        hex.push_str(&format!("{:02x}", bytes))
    }

    hex
}
