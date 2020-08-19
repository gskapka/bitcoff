use crate::lib::types::Result;

pub fn maybe_save_output(
    output: String,
    maybe_path: &Option<String>
) -> Result<String> {
    use std::fs;
    match maybe_path {
        None => Ok(output),
        Some(path) => {
            info!("✔ Saving output to {}", path);
            fs::write(path, output.clone())?;
            Ok(output)
        }
    }
}
