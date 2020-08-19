use std::process::Command;
use crate::lib::{
    state::State,
    types::Result,
    errors::AppError,
    btc_private_key::BtcPrivateKey,
    utils::{
        file_exists,
        convert_bytes_to_string_with_no_new_lines,
    },
};

fn check_keyfile_exists(keyfile_path: &str) -> Result<()> {
    info!("✔ Checking BTC private keyfile exists...");
    match file_exists(&keyfile_path) {
        false =>
            Err(AppError::Custom(
                format!(
                    "✘ BTC keyfile path: '{}' not found!\n{}",
                    keyfile_path,
                    "✘ Please create a keyfile - see `README.md for details"
                )
            )),
        true => {
            info!("✔ Key file found @ {}!", keyfile_path);
            Ok(())
        }
    }
}

fn maybe_get_btc_private_key_wif_string(keyfile_path: &str) -> Result<String> {
    info!("✔ Decrypting private key...");
    let output = Command::new("gpg").arg("-d").arg(keyfile_path).output()?;
    match output.stdout.len() {
        0 => {
            info!("✘ Error decrypting keyfile!");
            Err(AppError::Custom(convert_bytes_to_string_with_no_new_lines(&output.stderr)?))
        }
        _ => {
            info!("✔ Keyfile decrypted!");
            convert_bytes_to_string_with_no_new_lines(&output.stdout)
        }
    }
}

fn get_btc_private_key_from_wif(btc_pk_wif: String) -> Result<BtcPrivateKey> {
    info!("✔ Creating BTC private key from WIF...");
    let btc_pk = BtcPrivateKey::from_wif(&btc_pk_wif)?;
    info!("✔ BTC address: '{}'", btc_pk.to_p2pkh_btc_address());
    Ok(btc_pk)
}

pub fn get_btc_private_key_and_add_to_state(state: State) -> Result<State> {
    info!("✔ Maybe getting BTC private key & adding to state...");
    let keyfile_path = &state.cli_args.flag_keyfile;
    check_keyfile_exists(keyfile_path)
        .and_then(|_| maybe_get_btc_private_key_wif_string(keyfile_path))
        .and_then(get_btc_private_key_from_wif)
        .and_then(|btc_private_key| state.add_btc_private_key(btc_private_key))
}
