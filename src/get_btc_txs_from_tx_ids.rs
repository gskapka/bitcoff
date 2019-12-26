use reqwest;
use bitcoin::{
    consensus::encode::deserialize as btc_deserialize,
    blockdata::transaction::Transaction as BtcTransaction,
};
use crate::{
    state::State,
    types::{
        Result,
        BtcTransactions,
    },
    errors::AppError,
};

pub fn convert_hex_tx_to_btc_tx(hex: &String) -> Result<BtcTransaction> {
    info!("✔ Converting hex to BTC tx...");
    Ok(btc_deserialize::<BtcTransaction>(&hex::decode(hex)?)?)
}

fn convert_hex_txs_to_btc_txs(hex_txs: Vec<String>) -> Result<BtcTransactions> {
    info!("✔ Converting hex txs to BTC txs...");
    hex_txs
        .iter()
        .map(convert_hex_tx_to_btc_tx)
        .collect::<Result<BtcTransactions>>()
}

fn get_hex_tx_from_tx_id(
    tx_id: &String,
    api_endpoint: &String,
) -> Result<String> {
    info!("✔ Getting BTC tx in hex format for tx id: {}", tx_id);
    match reqwest::get(
        &format!("{}tx/{}/hex", api_endpoint, tx_id)[..]
    ) { 
        Err(e) => Err(AppError::Custom(e.to_string())),
        Ok(mut body) => match body.status() {
            reqwest::StatusCode::OK => {
                match body.text() {
                    Ok(text) => Ok(text),
                    Err(e) => Err(AppError::Custom(e.to_string()))
                }
            }
            _ => {
                debug!( "✘ Error getting BTC tx in hex: {:?}", body);
                Err(AppError::Custom(
                    format!(
                        "✘ Error getting BTC tx in hex, status code: {}", 
                        body.status()
                    )
                ))
            }
        }
    }
}

fn get_hex_txs_from_tx_ids(
    tx_ids: &Vec<String>,
    api_endpoint: &String,
) -> Result<Vec<String>> {
    info!("✔ Getting BTC txs in hex format...");
    tx_ids
        .iter()
        .map(|tx_id| get_hex_tx_from_tx_id(tx_id, api_endpoint))
        .collect::<Result<Vec<String>>>()
}

pub fn maybe_get_txs_from_tx_ids_and_put_in_state(
    state: State
) -> Result<State> {
    info!("✔ Maybe getting BTC txs...");
    get_hex_txs_from_tx_ids(
        &state.cli_args.arg_tx_id, 
        &state.api_endpoint,
    )
        .and_then(convert_hex_txs_to_btc_txs)
        .and_then(|btc_tx| state.add_btc_txs(btc_tx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        get_sample_tx_hex,
        SAMPLE_TESTNET_TX_ID,
        SAMPLE_TESTNET_ENDPOINT,
    };

    #[test]
    fn should_get_tx_in_hex_format() {
        match get_hex_tx_from_tx_id(
            &SAMPLE_TESTNET_TX_ID.to_string(),
            &SAMPLE_TESTNET_ENDPOINT.to_string(),
        ) {
            Err(e) => panic!("Error getting tx id: {}", e),
            Ok(result) => assert!(result == get_sample_tx_hex()),
        }
    }

    #[test]
    fn should_convert_hex_to_btc_tx() {
        if let Err(e) = convert_hex_tx_to_btc_tx(
            &get_sample_tx_hex().to_string()
        ) {
            panic!("Error converting hex to btc tx: {}", e);
        }
    }
}
