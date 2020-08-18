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

pub type Byte = u8;
pub type Bytes = Vec<Byte>;
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

impl BtcUtxoAndValueJson {
    pub fn from_utxo_and_value(
        utxo_and_value: &BtcUtxoAndValue
    ) -> Self {
        BtcUtxoAndValueJson {
            utxo_value: utxo_and_value.value,
            utxo_hex: hex::encode(utxo_and_value.serialized_utxo.clone()),
        }
    }

    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
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

    pub fn from_json(json: &str) -> Result<Self> {
        BtcUtxoAndValueJson::from_json(json)
            .and_then(|utxo_json| Ok(Self::new_serialized(utxo_json.utxo_value, hex::decode(&utxo_json.utxo_hex)?)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::SAMPLE_UTXO_JSON_STRING;

    #[test]
    fn should_get_btc_utxo_and_value_json_from_json_string() {
        if let Err(e) = BtcUtxoAndValueJson::from_json(SAMPLE_UTXO_JSON_STRING) {
            panic!("Error getting `BtcUtxoAndValueJson` from json: {}", e);
        }
    }

    #[test]
    fn should_get_btc_utxo_and_value_from_json() {
        if let Err(e) = BtcUtxoAndValue::from_json(SAMPLE_UTXO_JSON_STRING) {
            panic!("Error getting `BtcUtxoAndValue` from json: {}", e);
        }
    }
}
