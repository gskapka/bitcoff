use std::result;
use serde_json::{
    json,
    Value as JsonValue,
};
use crate::lib::{
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
pub type BtcAddressesAndAmounts = Vec<(String, u64)>;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UtxoInfo {
    pub vout: u32,
    pub value: u64,
    pub txid: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BtcUtxosAndValues(pub Vec<BtcUtxoAndValue>);

impl BtcUtxosAndValues {
    pub fn from_vec(vec: Vec<BtcUtxoAndValue>) -> Self {
        BtcUtxosAndValues(vec)
    }

    pub fn to_vec(&self) -> Vec<BtcUtxoAndValue> {
        self.0.to_vec()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_utxos(&self) -> Vec<BtcUtxo> {
        self.to_vec().iter().map(|x| x.utxo.clone()).collect()
    }

    pub fn get_total_value(&self) -> u64 {
        self.to_vec().iter().map(|x| x.value).sum()
    }

    pub fn from_json(json: &str) -> Result<Self> {
        let strings: Vec<JsonValue> = serde_json::from_str(json)?;
        Ok(
            BtcUtxosAndValues(
                strings
                    .iter()
                    .map(|json_value| BtcUtxoAndValue::from_json(&json_value.to_string()))
                    .collect::<Result<Vec<BtcUtxoAndValue>>>()?
                )
        )
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self.0.iter().map(BtcUtxoAndValue::to_json_value).collect::<Vec<JsonValue>>())?)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BtcUtxoAndValue {
    pub value: u64,
    pub utxo: BtcUtxo,
}

impl BtcUtxoAndValue {
    pub fn new(value: u64, utxo: &BtcUtxo) -> Self {
        BtcUtxoAndValue { value, utxo: utxo.clone() }
    }

    pub fn from_json(json: &str) -> Result<Self> {
        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
        pub struct IntermediateJson {
            pub value: u64,
            pub serialized_utxo: String,
        };
        let json: IntermediateJson = serde_json::from_str(json)?;
        Ok(Self::new(json.value, &deserialize_btc_utxo(&hex::decode(json.serialized_utxo)?)?))
    }

    pub fn to_json_value(&self) -> JsonValue {
        json!({
            "value": self.value,
            "serialized_utxo": hex::encode(&serialize_btc_utxo(&self.utxo)),
        })
    }

    #[cfg(test)]
    pub fn to_json(&self) -> String {
        self.to_json_value().to_string()
    }

    #[cfg(test)]
    fn get_serialized_utxo(&self) -> Bytes {
        serialize_btc_utxo(&self.utxo)
    }

    #[cfg(test)]
    pub fn get_serialized_utxo_hex(&self) -> String {
        hex::encode(self.get_serialized_utxo())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::test_utils::{
        get_sample_utxo,
        SAMPLE_UTXO_JSON_STRING,
    };

    #[test]
    fn should_get_btc_utxo_and_value_from_json() {
        if let Err(e) = BtcUtxoAndValue::from_json(SAMPLE_UTXO_JSON_STRING) {
            panic!("Error getting `BtcUtxoAndValue` from json: {}", e);
        }
    }

    #[test]
    fn should_perform_serde_json_round_trip() {
        let utxo = BtcUtxoAndValue::from_json(SAMPLE_UTXO_JSON_STRING).unwrap();
        let json = utxo.to_json();
        let result = BtcUtxoAndValue::from_json(&json).unwrap();
        assert_eq!(result, utxo);
    }

    #[test]
    fn should_make_btc_utxo_and_values_serde_json_round_trip_correctly() {
        let utxo_1 = get_sample_utxo();
        let utxo_2 = utxo_1.clone();
        let utxos = BtcUtxosAndValues(vec![utxo_1, utxo_2]);
        let json = utxos.to_json().unwrap();
        let result = BtcUtxosAndValues::from_json(&json).unwrap();
        assert_eq!(utxos, result);
    }
}
