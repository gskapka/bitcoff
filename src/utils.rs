use secp256k1::SecretKey;
use rand::{
    RngCore,
    thread_rng,
};
use crate::{
    types::{
        Bytes,
        Result,
    },
};

pub fn bytes_to_utf8_str(bytes: &Bytes) -> Result<String> {
    Ok(std::str::from_utf8(bytes)?.to_string())
}

pub fn strip_new_lines_from_str(string: String) -> String {
    string.replace("\n", "")
}

pub fn convert_bytes_to_string_with_no_new_lines(
    bytes: &Bytes
) -> Result<String> {
    bytes_to_utf8_str(bytes)
        .map(strip_new_lines_from_str)
}

pub fn file_exists(path: &String) -> bool {
    std::path::Path::new(path).is_file()
}

pub fn generate_random_private_key() -> Result<SecretKey> {
    Ok(SecretKey::from_slice(&get_32_random_bytes_arr())?)
}

pub fn get_32_random_bytes_arr() -> [u8; 32] {
    let mut arr = [0; 32];
    arr.copy_from_slice(&get_x_random_bytes(32));
    arr
}

fn get_x_random_bytes(num_bytes: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; num_bytes];
    thread_rng().fill_bytes(&mut bytes);
    bytes
}
