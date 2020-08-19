use bitcoin::blockdata::transaction::{
    TxOut as BtcTxOut,
    Transaction as BtcTransaction,
};
use crate::lib::{
    types::BtcUtxoAndValue,
    btc_private_key::BtcPrivateKey,
    get_btc_txs_from_utxos_info::convert_hex_tx_to_btc_tx,
};

pub const SAMPLE_UTXO_INDEX: u32 = 0;
pub const SAMPLE_TESTNET_ENDPOINT: &str = "https://blockstream.info/testnet/api/";
pub const SAMPLE_TARGET_BTC_ADDRESS: &str = "moBSQbHn7N9BC9pdtAMnA7GBiALzNMQJyE";
pub const SAMPLE_BTC_PRIVATE_KEY: &str = "cP2Dv4mx1DwJzN8iF6CCyPZmuS27bT9MV4Qmgb9h6cNQNq2Jgpmy";
pub const SAMPLE_TESTNET_TX_ID: &str = "85f8faf4a3da404a833d0a21b1cea215da74a4b2c1ce8187cbf6379f42c02924";
pub const SAMPLE_BTC_PUBLIC_KEY: &str = "03d2a5e3b162eb580fe2ce023cd5e0dddbb6286923acde77e3e5468314dc9373f7";
pub const SAMPLE_UTXOS_JSON_STRING: &str = "[{\"value\":891168,\"serialized_utxo\":\"6e3fa15afcd9b579b7ed082e0ee8cfba1f27a6cf007cb7ca95b06ab0fda2880c020000001976a91454102783c8640c5144d039cea53eb7dbb470081488acffffffff\"}]";
pub const SAMPLE_UTXO_JSON_STRING: &str = "{\"value\":891168,\"serialized_utxo\":\"6e3fa15afcd9b579b7ed082e0ee8cfba1f27a6cf007cb7ca95b06ab0fda2880c020000001976a91454102783c8640c5144d039cea53eb7dbb470081488acffffffff\"}";

pub fn get_sample_btc_private_key() -> BtcPrivateKey {
    BtcPrivateKey::from_wif(SAMPLE_BTC_PRIVATE_KEY).unwrap()
}

pub fn get_sample_tx_hex() -> &'static str {
    "01000000018986374e3404c889f3da5fd8b07311cad5b0e81e333a994638f65c9a9cdf4742010000006a47304402201db6cfd4be08ed4605b5eed60281438ea325af6ea6f0ff7e19f46431c29fcbcb0220157d5a1773f5eaff369735ea7608fd31b603fe279a45b5ee2f5d555c25711566012103d8d40098fa07622a89491597be95836a05de0fa5fcca1e474eb6a6213fc1f33fffffffff0282060000000000001976a91454102783c8640c5144d039cea53eb7dbb470081488acb4b81b01000000001976a9148302e646c0d9bf8b7292c6da11a721149e06749d88ac00000000"
}

pub fn get_sample_tx() -> BtcTransaction {
    convert_hex_tx_to_btc_tx(&get_sample_tx_hex().to_string()).unwrap()
}

pub fn get_sample_tx_output() -> BtcTxOut {
    get_sample_tx().output[SAMPLE_UTXO_INDEX as usize].clone()
}

pub fn get_sample_utxo() -> BtcUtxoAndValue {
    BtcUtxoAndValue::from_json(SAMPLE_UTXO_JSON_STRING).unwrap()
}
