use crate::lib::{
    state::State,
    types::{
        Result,
        UtxosInfo,
    },
    errors::AppError,
    utils::make_api_call,
};

fn get_utxo_info_json_string(address: &str, api_endpoint: &str) -> Result<String> {
    info!("✔ Getting UTXO info for address: {}", address);
    make_api_call(&format!("{}address/{}/utxo", api_endpoint, address)[..], "✘ Error getting UTXO list")
}

fn parse_utxo_list_json_string(utxo_list_json_string: &str) -> Result<UtxosInfo> {
    info!("✔ Parsing UTXO list JSON string...");
    match serde_json::from_str(&utxo_list_json_string) {
        Ok(json) => Ok(json),
        Err(e) => Err(AppError::Custom(e.to_string()))
    }
}

pub fn get_utxos_info_and_add_to_state(state: State) -> Result<State> {
    info!("✔ Getting UTXOs info and adding to state...");
    get_utxo_info_json_string(&state.get_btc_address()?, &state.api_endpoint)
        .and_then(|json| parse_utxo_list_json_string(&json))
        .and_then(|utxos_info| {
            info!("✔ {} UTXO(s) in list", utxos_info.len());
            state.add_utxos_info(utxos_info)
        })
}

pub fn get_utxos_info_for_address_in_cli_args_and_add_to_state(state: State) -> Result<State> {
    info!("✔ Getting UTXOs info for address in CLI args and adding to state...");
    get_utxo_info_json_string(&state.cli_args.arg_btcAddress, &state.api_endpoint)
        .and_then(|json| parse_utxo_list_json_string(&json))
        .and_then(|utxos_info| {
            info!("✔ {} UTXO(s) in list", utxos_info.len());
            state.add_utxos_info(utxos_info)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::test_utils::{
        SAMPLE_TESTNET_ENDPOINT,
        SAMPLE_TARGET_BTC_ADDRESS,
    };

    #[test]
    fn should_get_utxo_list_json_string() {
        let result = get_utxo_info_json_string(
            &SAMPLE_TARGET_BTC_ADDRESS.to_string(),
            &SAMPLE_TESTNET_ENDPOINT.to_string(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn should_parse_utxo_list_json_string() {
        let utxo_list_json_string = get_utxo_info_json_string(
            &SAMPLE_TARGET_BTC_ADDRESS.to_string(),
            &SAMPLE_TESTNET_ENDPOINT.to_string(),
        ).unwrap();
        let result = parse_utxo_list_json_string(&utxo_list_json_string);
        assert!(result.is_ok());
    }
}
