use std::result;
use crate::{
    errors::AppError,
    utils::{
        serialize_btc_utxo,
        deserialize_btc_utxo,
    },
};
use bitcoin::{
    util::address::Address as BtcAddress,
    blockdata::{
        block::Block as BtcBlock,
        transaction::{
            TxIn as BtcUtxo,
            Transaction as BtcTransaction,
        },
    },
};

pub type Bytes = Vec<u8>;
pub type UtxosInfo = Vec<UtxoInfo>;
pub type BtcTransactions = Vec<BtcTransaction>;
pub type Result<T> = result::Result<T, AppError>;
pub type BtcUtxosAndValues = Vec<BtcUtxoAndValue>;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UtxoInfo {
    pub value: u64,
    pub vout: usize,
    pub txid: String,
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
