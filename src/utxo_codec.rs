use crate::{
    state::State,
    types::{
        Result,
        BtcUtxoAndValue,
        BtcUtxosAndValues,
        BtcUtxoAndValueJson,
        BtcUtxoAndValueJsons,
    },
};

fn convert_utxo_and_values_to_jsons(utxos: &[BtcUtxoAndValue]) -> Vec<BtcUtxoAndValueJson> {
    utxos.iter().map(|utxo| BtcUtxoAndValueJson::from_utxo_and_value(utxo)).collect()
}

fn convert_utxos_json_string_to_utxos_jsons(utxo_json_string: &str) -> Result<BtcUtxoAndValueJsons> {
    Ok(serde_json::from_str(utxo_json_string)?)
}

fn convert_utxo_json_to_utxo(utxo_and_value_json: &BtcUtxoAndValueJson) -> Result<BtcUtxoAndValue> {
    Ok(BtcUtxoAndValue::new_serialized(utxo_and_value_json.utxo_value, hex::decode(&utxo_and_value_json.utxo_hex)?))
}

fn convert_utxo_jsons_to_utxos(utxo_jsons: BtcUtxoAndValueJsons) -> Result<BtcUtxosAndValues> {
    utxo_jsons.iter().map(convert_utxo_json_to_utxo).collect::<Result<BtcUtxosAndValues>>()
}

pub fn get_utxos_from_utxo_json_string_and_add_to_state(state: State) -> Result<State> {
    convert_utxos_json_string_to_utxos_jsons(&state.get_utxo_json_string()?)
        .and_then(convert_utxo_jsons_to_utxos)
        .and_then(|utxos| state.add_btc_utxos_and_values(utxos))
}

pub fn get_utxo_json_string_from_utxos_in_state(state: State) -> Result<String> {
    Ok(serde_json::to_string(&convert_utxo_and_values_to_jsons(state.get_btc_utxos_and_values()?))?)
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::test_utils::SAMPLE_UTXO_JSON_STRING;

    fn get_sample_utxo_json() -> BtcUtxoAndValueJson {
        convert_utxos_json_string_to_utxos_jsons(SAMPLE_UTXO_JSON_STRING).unwrap()[0].clone()
    }

    #[test]
    fn should_convert_utxo_json_string_to_utxos_and_values() {
        if let Err(e) = convert_utxos_json_string_to_utxos_jsons(
            SAMPLE_UTXO_JSON_STRING
        ) {
            panic!("Error converting utxo json string to utxo json: {}", e);
        }
    }

    #[test]
    fn should_convert_utxo_and_value_json_to_utxo_and_value() {
        let utxo_json = get_sample_utxo_json();
        if let Err(e) = convert_utxo_json_to_utxo(&utxo_json) {
            panic!("Error convert utxo json to utxo: {}", e);
        }
    }

    #[test]
    fn should_test_utxo_json_serde_round_trip() {
        let utxo_json = get_sample_utxo_json();
        let utxo = convert_utxo_json_to_utxo(&utxo_json).unwrap();
        let result = BtcUtxoAndValueJson::from_utxo_and_value(&utxo);
        assert!(result == utxo_json);
    }
}
