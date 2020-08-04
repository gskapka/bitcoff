use std::result;
use crate::{
    errors::AppError,
    utils::{
        serialize_btc_utxo,
        deserialize_btc_utxo,
    },
};
use bitcoin::blockdata::transaction::{
    TxIn as BtcUtxo,
    Transaction as BtcTransaction,
};

pub type Bytes = Vec<u8>;
pub type UtxosInfo = Vec<UtxoInfo>;
pub type BtcTransactions = Vec<BtcTransaction>;
pub type Result<T> = result::Result<T, AppError>;
pub type BtcUtxosAndValues = Vec<BtcUtxoAndValue>;
pub type BtcAddressesAndAmounts = Vec<(String, u64)>;
pub type BtcUtxoAndValueJsons = Vec<BtcUtxoAndValueJson>;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UtxoInfo {
    pub vout: u32,
    pub value: u64,
    pub txid: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct BtcUtxoAndValueJson {
    pub utxo_value: u64,
    pub utxo_hex: String,
}

impl BtcUtxoAndValueJson { // TODO test!
    pub fn from_utxo_and_value(
        utxo_and_value: &BtcUtxoAndValue
    ) -> Self {
        BtcUtxoAndValueJson {
            utxo_value: utxo_and_value.value,
            utxo_hex: hex::encode(utxo_and_value.serialized_utxo.clone()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct BtcUtxoAndValue {
    pub value: u64,
    pub serialized_utxo: Bytes,
}

impl BtcUtxoAndValue {
    pub fn new(value: u64, utxo: &BtcUtxo) -> Self {
        BtcUtxoAndValue {
            value,
            serialized_utxo: serialize_btc_utxo(utxo),
        }
    }

    pub fn new_serialized(value: u64, serialized_utxo: Bytes) -> Self {
        BtcUtxoAndValue { serialized_utxo, value }
    }

    pub fn get_utxo(&self) -> Result<BtcUtxo> {
        deserialize_btc_utxo(&self.serialized_utxo)
    }
}
