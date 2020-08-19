use crate::lib::{
    state::State,
    types::{
        Result,
        BtcUtxosAndValues,
    },
};

pub fn get_utxos_from_utxo_json_string_and_add_to_state(state: State) -> Result<State> {
    BtcUtxosAndValues::from_json(state.get_utxo_json_string()?)
        .and_then(|x| state.add_btc_utxos_and_values(x))
}

pub fn get_utxo_json_string_from_utxos_in_state(state: State) -> Result<String> {
    state.get_btc_utxos_and_values()?.to_json()
}
