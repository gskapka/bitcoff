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

fn convert_hex_tx_to_btc_tx(hex: &String) -> Result<BtcTransaction> {
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
        SAMPLE_TESTNET_ENDPOINT
    };

    fn get_sample_tx_hex() -> &'static str {
        "0200000001c351e079d6295d583d18c8b402310c917964d0c2cb71f57cb98590d92531ef66000000006a47304402207e34334ea611af33ad8a74126fe09b6cf870aefc99d64481c8e68ad2074aac9e02201de15364d45bf34597fa5d39583118764c8102519f655269c2e96db97fe3f4b9012103c9168357c5b9758c06d27428fe82f9b646c8843e19f93cc15aafd07fc2be0142ffffffff0242a5df01000000001976a9147bbd5aea02f4c08ab614523f2f256d4cd2b308cf88acc8000000000000001976a914c1e124adb43f6676739d40d93998ab8476ee46c188ac00000000"
    }

    #[test]
    fn should_get_tx_in_hex_format() {
        let tx_id = 
            "43e2ffe75efb144938dff8460b3ac7e12eb86f05f38f267c699aa46f32953ba8";
        match get_hex_tx_from_tx_id(
            &tx_id.to_string(),
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
