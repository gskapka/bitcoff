use serde_json;
use crate::{
    state::State,
    types::{
        Result,
        BtcUtxosAndValues,
        BtcUtxoAndValueJson,
    },
};

fn convert_utxo_and_values_to_jsons(
    utxos: &BtcUtxosAndValues
) -> Vec<BtcUtxoAndValueJson> {
    utxos
        .iter()
        .map(|utxo| BtcUtxoAndValueJson::from_utxo_and_value(utxo))
        .collect::<Vec<BtcUtxoAndValueJson>>()
}

pub fn get_utxo_json_string_from_utxos_in_state(
    state: State
) -> Result<String> {
    Ok(
        serde_json::to_string(
            &convert_utxo_and_values_to_jsons(
                state.get_btc_utxos_and_values()?
            )
        )?
    )
}
