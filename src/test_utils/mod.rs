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
    get_btc_txs_from_tx_ids::convert_hex_tx_to_btc_tx,
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

pub const SAMPLE_UTXO_INDEX: u32 = 0;

pub const SAMPLE_TESTNET_ENDPOINT: &'static str =
    "https://blockstream.info/testnet/api/";

pub const SAMPLE_BTC_PRIVATE_KEY: &'static str =
    "cP2Dv4mx1DwJzN8iF6CCyPZmuS27bT9MV4Qmgb9h6cNQNq2Jgpmy";

pub const SAMPLE_TARGET_BTC_ADDRESS: &'static str = // TODO RENAME!
    "moBSQbHn7N9BC9pdtAMnA7GBiALzNMQJyE";

pub const SAMPLE_BTC_PUBLIC_KEY: &'static str =
    "03d2a5e3b162eb580fe2ce023cd5e0dddbb6286923acde77e3e5468314dc9373f7";

pub const SAMPLE_TESTNET_TX_ID: &'static str =
    "85f8faf4a3da404a833d0a21b1cea215da74a4b2c1ce8187cbf6379f42c02924";

pub fn get_sample_btc_private_key() -> BtcPrivateKey {
    BtcPrivateKey::from_wif(SAMPLE_BTC_PRIVATE_KEY)
        .unwrap()
}

pub fn get_sample_tx_hex() -> &'static str {
    "01000000018986374e3404c889f3da5fd8b07311cad5b0e81e333a994638f65c9a9cdf4742010000006a47304402201db6cfd4be08ed4605b5eed60281438ea325af6ea6f0ff7e19f46431c29fcbcb0220157d5a1773f5eaff369735ea7608fd31b603fe279a45b5ee2f5d555c25711566012103d8d40098fa07622a89491597be95836a05de0fa5fcca1e474eb6a6213fc1f33fffffffff0282060000000000001976a91454102783c8640c5144d039cea53eb7dbb470081488acb4b81b01000000001976a9148302e646c0d9bf8b7292c6da11a721149e06749d88ac00000000"
}

pub fn get_sample_tx() -> BtcTransaction {
    convert_hex_tx_to_btc_tx(&get_sample_tx_hex().to_string())
        .unwrap()
}

pub fn get_sample_tx_output() -> BtcTxOut {
    get_sample_tx().output[SAMPLE_UTXO_INDEX as usize].clone()
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
