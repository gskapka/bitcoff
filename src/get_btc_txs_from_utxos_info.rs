use bitcoin::{
    consensus::encode::deserialize as btc_deserialize,
    blockdata::transaction::Transaction as BtcTransaction,
};
use crate::{
    state::State,
    utils::make_api_call,
    types::{
        Result,
        UtxosInfo,
        BtcTransactions,
    },
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
    make_api_call(
        &format!("{}tx/{}/hex", api_endpoint, tx_id)[..],
        &"✘ Error getting BTC tx in hex: {:?}",
    )
}

fn get_hex_txs_from_utxos_info(
    utxos_info: &UtxosInfo,
    api_endpoint: &String,
) -> Result<Vec<String>> {
    info!("✔ Getting BTC txs in hex format...");
    utxos_info
        .iter()
        .map(|utxo_info| get_hex_tx_from_tx_id(&utxo_info.txid, api_endpoint))
        .collect::<Result<Vec<String>>>()
}

pub fn get_txs_from_utxo_infos_and_put_in_state(
    state: State
) -> Result<State> {
    info!("✔ Maybe getting BTC txs...");
    get_hex_txs_from_utxos_info(
        state.get_utxos_info()?,
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
