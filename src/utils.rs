use secp256k1::SecretKey;
use rand::{
    RngCore,
    thread_rng,
};
use crate::{
    types::{
        Bytes,
        Result,
        BtcUtxosAndValues,
    },
    base58::{
        from as from_base58,
    },
};
use bitcoin::{
    consensus::encode::serialize as btc_serialize,
    consensus::encode::deserialize as btc_deserialize,
    blockdata::{
        opcodes,
        block::Block as BtcBlock,
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

pub fn convert_btc_address_to_pub_key_hash_bytes(
    btc_address: &str
) -> Result<Bytes> {
    Ok(from_base58(btc_address)?[1..21].to_vec())
}

pub fn serialize_btc_utxo(btc_utxo: &BtcUtxo) -> Bytes {
    btc_serialize(btc_utxo)
}

pub fn deserialize_btc_utxo(bytes: &Bytes) -> Result<BtcUtxo> {
    Ok(btc_deserialize(bytes)?)
}

pub fn get_total_value_of_utxos_and_values(
    utxos_and_values: &BtcUtxosAndValues
) -> u64 {
   utxos_and_values
        .iter()
        .map(|utxo_and_value| utxo_and_value.value)
        .sum()
}

pub fn get_pay_to_pub_key_hash_script(btc_address: &str) -> Result<BtcScript> {
    let script = BtcScriptBuilder::new();
    Ok(
        script
            .push_opcode(opcodes::all::OP_DUP)
            .push_opcode(opcodes::all::OP_HASH160)
            .push_slice(
                &convert_btc_address_to_pub_key_hash_bytes(btc_address)?[..]
            )
            .push_opcode(opcodes::all::OP_EQUALVERIFY)
            .push_opcode(opcodes::all::OP_CHECKSIG)
            .into_script()
    )
}

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
