use reqwest;
use crate::{
    state::State,
    types::{
        Result,
        UtxosInfo,
    },
    errors::AppError,
    utils::make_api_call,
};

fn get_utxo_list_json_string(
    address: &String,
    api_endpoint: &String,
) -> Result<String> {
    info!("✔ Getting UTXO list for address: {}", address);
    make_api_call(
        &format!("{}address/{}/utxo", api_endpoint, address)[..],
        "✘ Error getting UTXO list",
    )
}

fn parse_utxo_list_json_string(
    utxo_list_json_string: String
) -> Result<UtxosInfo> {
    info!("✔ Parsing UTXO list JSON string...");
    match serde_json::from_str(&utxo_list_json_string) {
        Ok(json) => Ok(json),
        Err(e) => Err(AppError::Custom(e.to_string()))
    }
}

pub fn get_utxos_info_and_add_to_state(state: State) -> Result<State> {
    info!("✔ Getting UTXOs info and adding to state...");
    get_utxo_list_json_string(
        &state.api_endpoint,
        &state.get_btc_address()?,
    )
        .and_then(parse_utxo_list_json_string)
        .and_then(|utxos_info| state.add_utxos_info(utxos_info))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        SAMPLE_TESTNET_ENDPOINT,
        SAMPLE_TARGET_BTC_ADDRESS,
    };

    #[test]
    fn should_get_utxo_list_json_string() {
        if let Err(e) = get_utxo_list_json_string(
            &SAMPLE_TARGET_BTC_ADDRESS.to_string(),
            &SAMPLE_TESTNET_ENDPOINT.to_string(),
        ) {
            panic!("Error getting utxo list: {}", e);
        };
    }

    #[test]
    fn should_parse_utxo_list_json_string() {
        let utxo_list_json_string = get_utxo_list_json_string(
            &SAMPLE_TARGET_BTC_ADDRESS.to_string(),
            &SAMPLE_TESTNET_ENDPOINT.to_string(),
        ).unwrap();
        let result = parse_utxo_list_json_string(utxo_list_json_string)
            .unwrap();
        println!("{:?}", result);
    }
}
