use std::fs::read_to_string;
use bitcoin::blockdata::{
    script::Script as BtcScript,
    transaction::{
        TxIn as BtcUtxo,
        TxOut as BtcTxOut,
        OutPoint as BtcOutPoint,
        Transaction as BtcTransaction,
    },
};
use crate::{
    types::Result,
    errors::AppError,
    btc_private_key::BtcPrivateKey,
    /*
    extract_utxos_from_btc_txs::create_btc_utxo_and_value_from_tx_output,
    btc_utils::get_pay_to_pub_key_hash_script,
    btc_types::{
        BtcBlockAndId,
        BtcUtxoAndValue,
        BtcBlockAndTxsJson,
    },
    parse_btc_block::{
        parse_btc_block_string_to_json,
        parse_btc_block_and_tx_json_to_struct,
    },
    */
};

pub const SAMPLE_BTC_PRIVATE_KEY: &'static str =
    "cP2Dv4mx1DwJzN8iF6CCyPZmuS27bT9MV4Qmgb9h6cNQNq2Jgpmy";

pub const SAMPLE_TARGET_BTC_ADDRESS: &'static str =
    "moBSQbHn7N9BC9pdtAMnA7GBiALzNMQJyE";

pub const SAMPLE_BTC_PUBLIC_KEY: &'static str =
    "03d2a5e3b162eb580fe2ce023cd5e0dddbb6286923acde77e3e5468314dc9373f7";

pub fn get_sample_btc_private_key() -> BtcPrivateKey {
    BtcPrivateKey::from_wif(SAMPLE_BTC_PRIVATE_KEY)
        .unwrap()
}

/*
pub const SAMPLE_TRANSACTION_INDEX: usize = 1;
pub const SAMPLE_BTC_UTXO_VALUE: u64 = 3347338;
pub const SAMPLE_OUTPUT_INDEX_OF_UTXO: u32 = 0;
pub const SAMPLE_TRANSACTION_OUTPUT_INDEX: usize = 0;
pub const SAMPLE_OP_RETURN_TRANSACTION_INDEX: usize = 56;
pub const SAMPLE_OP_RETURN_TRANSACTION_OUTPUT_INDEX: usize = 1;

pub const SAMPLE_BTC_BLOCK_JSON_PATH: &str =
    "src/btc/test_utils/604700-btc-block-and-txs.json";

pub const SAMPLE_TESTNET_BTC_BLOCK_JSON_PATH: &str =
    "src/btc/test_utils/1610046-testnet-block-with-tx-to-test-address.json";

pub const SAMPLE_TESTNET_BTC_BLOCK_JSON_PATH2: &str =
    "src/btc/test_utils/1610166-testnet-block-with-tx-to-test-address.json";

pub const SAMPLE_TESTNET_BTC_BLOCK_JSON_PATH3: &str =
    "src/btc/test_utils/1610161-testnet-block-with-tx-to-test-address.json";

pub const SAMPLE_TESTNET_OP_RETURN_BTC_BLOCK_JSON: &str =
    "src/btc/test_utils/1610826-testnet-block-with-tx-to-test-address.json";

pub const SAMPLE_SERIALIZED_BTC_UTXO: &'static str = "0e8d588f88d5624148070a8cd79508da8cb76625e4fcdb19a5fc996aa843bf04000000001976a91454102783c8640c5144d039cea53eb7dbb470081488acffffffff";

pub fn get_sample_sequential_block_and_ids() -> Vec<BtcBlockAndId> {
    let start_num = 1611090;
    let path_prefix = "src/btc/test_utils/sequential_block_and_ids/";
    let path_suffix = "-btc-block-and-txs.json";
    let mut paths = Vec::new();
    for i in 0..11 {
        paths.push(
            format!("{}{}{}", path_prefix, start_num + i, path_suffix)
        )
    };
    paths
        .iter()
        .map(|path|
             read_to_string(path).unwrap()
        )
        .map(|json_string|
            parse_btc_block_string_to_json(&json_string)
                .and_then(parse_btc_block_and_tx_json_to_struct)
        )
        .collect::<Result<Vec<BtcBlockAndId>>>()
        .unwrap()
}

pub fn get_sample_btc_block_json_string() -> String {
    read_to_string(SAMPLE_BTC_BLOCK_JSON_PATH)
        .unwrap()
}

pub fn get_sample_btc_block_json() -> Result<BtcBlockAndTxsJson> {
    parse_btc_block_string_to_json(
        &get_sample_btc_block_json_string()
    )
}

pub fn get_sample_btc_block_and_id() -> Result<BtcBlockAndId> {
    parse_btc_block_and_tx_json_to_struct(
        get_sample_btc_block_json()
            .unwrap()
    )
}

pub fn get_sample_testnet_block_and_txs() -> Result<BtcBlockAndId> {
    parse_btc_block_string_to_json(
        &read_to_string(&SAMPLE_TESTNET_BTC_BLOCK_JSON_PATH).unwrap()
    )
        .and_then(parse_btc_block_and_tx_json_to_struct)
}

pub fn get_sample_btc_tx() -> BtcTransaction {
    get_sample_testnet_block_and_txs()
        .unwrap()
        .block
        .txdata[SAMPLE_TRANSACTION_INDEX]
        .clone()
}

pub fn get_sample_op_return_btc_block_and_txs() -> BtcBlockAndId {
    parse_btc_block_string_to_json(
        &read_to_string(&SAMPLE_TESTNET_OP_RETURN_BTC_BLOCK_JSON).unwrap()

    )
        .and_then(parse_btc_block_and_tx_json_to_struct)
        .unwrap()
}

pub fn get_sample_btc_op_return_tx() -> BtcTransaction {
    get_sample_op_return_btc_block_and_txs()
        .block
        .txdata[SAMPLE_OP_RETURN_TRANSACTION_INDEX]
        .clone()
}

pub fn get_sample_op_return_output() -> BtcTxOut {
    get_sample_btc_op_return_tx()
      .output[SAMPLE_OP_RETURN_TRANSACTION_OUTPUT_INDEX]
      .clone()
}

pub fn get_sample_btc_tx_output() -> BtcTxOut {
    get_sample_btc_tx()
        .output[SAMPLE_TRANSACTION_OUTPUT_INDEX]
        .clone()
}

pub fn get_sample_btc_utxo() -> BtcUtxo {
    let tx = get_sample_btc_tx();
    let outpoint = BtcOutPoint {
        txid: tx.txid(),
        vout: SAMPLE_TRANSACTION_OUTPUT_INDEX as u32,
    };
    BtcUtxo {
        witness: vec![], // NOTE: Array of byte arrays (empty for non-segwit).
        sequence: 4294967295, // NOTE: Unused so just "0xFFFFFFFF" hardcoded
        previous_output: outpoint,
        script_sig: get_sample_pay_to_pub_key_hash_script(),
    }
}

pub fn get_sample_utxo_and_value() -> BtcUtxoAndValue {
    create_btc_utxo_and_value_from_tx_output(
        &get_sample_btc_tx(),
        SAMPLE_OUTPUT_INDEX_OF_UTXO,
    )
}

pub fn get_sample_utxo_and_value_n(n: usize) -> Result<BtcUtxoAndValue> {
    // NOTE: Tuple = path on disk, block_index of utxo & output_index of utxo!
    let tuple = match n {
        2 => Ok((SAMPLE_TESTNET_BTC_BLOCK_JSON_PATH2, 18, 2)),
        3 => Ok((SAMPLE_TESTNET_BTC_BLOCK_JSON_PATH3, 28, 0)),
        4 => Ok((SAMPLE_TESTNET_BTC_BLOCK_JSON_PATH3, 28, 1)),
        _ => Err(AppError::Custom(
            "✘ Don't have sample for that number!"
                .to_string()
        ))
    }.unwrap();
    parse_btc_block_string_to_json(&read_to_string(&tuple.0)?)
        .and_then(parse_btc_block_and_tx_json_to_struct)
        .map(|block_and_id| block_and_id.block.txdata[tuple.1].clone())
        .map(|tx| create_btc_utxo_and_value_from_tx_output(&tx, tuple.2))
}

pub fn get_sample_pay_to_pub_key_hash_script() -> BtcScript {
    get_pay_to_pub_key_hash_script(SAMPLE_TARGET_BTC_ADDRESS)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_sample_sequential_block_and_ids() {
        get_sample_sequential_block_and_ids();
    }

    #[test]
    fn should_not_panic_getting_sample_btc_block_string() {
        get_sample_btc_block_json_string();
    }

    #[test]
    fn should_not_panic_getting_sample_btc_block_json() {
        get_sample_btc_block_json()
            .unwrap();
    }

    #[test]
    fn should_not_panic_getting_sample_btc_block() {
        get_sample_btc_block_and_id()
            .unwrap();
    }

    #[test]
    fn should_not_panic_getting_testnet_sample_block() {
        get_sample_testnet_block_and_txs()
            .unwrap();
    }
}
*/
