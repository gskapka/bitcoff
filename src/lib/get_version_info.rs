use crate::lib::types::Result;

pub fn get_version_info() -> Result<String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}
