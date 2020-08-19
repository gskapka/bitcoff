use crate::lib::{
    state::State,
    types::Result,
    errors::AppError,
};

pub fn get_utxo_json_string_from_cli_args_and_add_to_state(
    state: State
) -> Result<State> {
    info!("✔ Getting UTXO json string from CLI args...");
    match state.cli_args.arg_utxos.clone() {
        Some(utxo_json_string) => {
            info!("✔ UTXO json string passed in as argument...");
            state.add_utxo_json_string(utxo_json_string)
        }
        None => {
            info!("✔ UTXO json string passed in as argument...");
            use std::fs::read_to_string;
            match state.cli_args.flag_utxoFile.clone() {
                None => Err(AppError::Custom(
                    "✘ No UTXO JSON file path passed in!".to_string()
                )),
                Some(path) => {
                    info!("✔ Reading UTXO JSON from path: {}", path);
                    let json_string = read_to_string(path)?;
                    info!("✔ JSON string: {}", json_string);
                    state.add_utxo_json_string(json_string)
                }
            }
        }
    }
}
