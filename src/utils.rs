use std::str::FromStr;
use secp256k1::SecretKey;
use rand::{
    RngCore,
    thread_rng,
};
use crate::{
    state::State,
    errors::AppError,
    types::{
        Byte,
        Bytes,
        Result,
        BtcUtxosAndValues,
    },
    base58::{
        from as from_base58,
    },
};
use bitcoin::{
    util::address::Address as BtcAddress,
    consensus::encode::serialize as btc_serialize,
    consensus::encode::deserialize as btc_deserialize,
    blockdata::{
        opcodes,
        transaction::{
            TxIn as BtcUtxo,
            TxOut as BtcTxOut,
            Transaction as BtcTransaction,
        },
        script::{
            Script as BtcScript,
            Builder as BtcScriptBuilder,
        },
    },
};

pub fn get_change_address_from_cli_args_in_state(state: &State) -> Result<String> {
    info!("✔ Getting change-address from CLI args in state...");
    match &state.cli_args.flag_change[..] {
        "signer" => Ok(state.get_btc_address()?),
        _ => Ok(state.cli_args.flag_change.clone())
    }
}

pub fn serialize_btc_tx_to_hex(tx: &BtcTransaction) -> String {
    hex::encode(&btc_serialize(tx))
}

pub fn serialize_tx_in_state(state: State) -> Result<String> {
    state.get_btc_tx().map(serialize_btc_tx_to_hex)
}

pub fn make_api_call(url: &str, error_message: &str) -> Result<String> {
    match reqwest::get(url) {
        Err(e) => Err(AppError::Custom(e.to_string())),
        Ok(mut body) => match body.status() {
            reqwest::StatusCode::OK => match body.text() {
                Ok(text) => Ok(text),
                Err(e) => Err(AppError::Custom(e.to_string()))
            }
            _ => {
                debug!("{}: {:?}", error_message, body);
                Err(AppError::Custom(format!("{} - status code: {}", error_message, body.status())))
            }
        }
    }
}

pub fn create_new_tx_output(amount_in_satoshis : &u64, btc_address: &str) -> Result<BtcTxOut> {
    Ok(
        BtcTxOut {
            value: *amount_in_satoshis,
            script_pubkey: BtcAddress::from_str(btc_address)?.script_pubkey(),
        }
    )
}

pub fn get_script_sig<'a>(signature_slice: &'a[u8], utxo_spender_pub_key_slice: &'a[u8]) -> BtcScript {
    BtcScriptBuilder::new()
        .push_slice(&signature_slice)
        .push_slice(&utxo_spender_pub_key_slice)
        .into_script()
}

// NOTE: Assumes compressed keys and no multi-sigs!
pub fn calculate_btc_tx_size(num_inputs: usize, num_outputs: usize) -> u64 {
    ((num_inputs * 148) + (num_outputs * 34) + 10 + num_inputs) as u64
}

pub fn calculate_btc_tx_fee(num_inputs: usize, num_outputs: usize, sats_per_byte: usize) -> u64 {
    calculate_btc_tx_size(num_inputs, num_outputs) * sats_per_byte as u64
}

pub fn convert_btc_address_to_pub_key_hash_bytes(btc_address: &str) -> Result<Bytes> {
    Ok(from_base58(btc_address)?[1..21].to_vec())
}

pub fn serialize_btc_utxo(btc_utxo: &BtcUtxo) -> Bytes {
    btc_serialize(btc_utxo)
}

pub fn deserialize_btc_utxo(bytes: &[Byte]) -> Result<BtcUtxo> {
    Ok(btc_deserialize(bytes)?)
}

pub fn get_total_value_of_utxos_and_values(utxos_and_values: &BtcUtxosAndValues) -> u64 {
   utxos_and_values.0.iter().map(|utxo_and_value| utxo_and_value.value).sum()
}

pub fn get_op_return_output(op_return_bytes: &[Byte]) -> Result<BtcTxOut> {
    Ok(BtcTxOut { value: 0, script_pubkey: get_op_return_script(op_return_bytes)?  })
}

fn get_op_return_script(op_return_bytes: &[Byte]) -> Result<BtcScript> {
    Ok(BtcScriptBuilder::new().push_opcode(opcodes::all::OP_RETURN).push_slice(op_return_bytes).into_script())
}

pub fn bytes_to_utf8_str(bytes: &[Byte]) -> Result<String> {
    Ok(std::str::from_utf8(bytes)?.to_string())
}

pub fn strip_new_lines_from_str(string: String) -> String {
    string.replace("\n", "")
}

pub fn convert_bytes_to_string_with_no_new_lines(bytes: &[Byte]) -> Result<String> {
    bytes_to_utf8_str(bytes).map(strip_new_lines_from_str)
}

pub fn file_exists(path: &str) -> bool {
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
