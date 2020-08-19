use bitcoin::blockdata::transaction::{
    TxIn as BtcUtxo,
    OutPoint as BtcOutPoint,
    Transaction as BtcTransaction,
};
use crate::lib::{
    state::State,
    constants::{
        ONE_BTC,
        DEFAULT_BTC_SEQUENCE,
    },
    types::{
        Result,
        UtxoInfo,
        BtcUtxoAndValue,
        BtcUtxosAndValues,
    },
};

pub fn create_btc_utxo_and_value_from_tx_output(tx: &BtcTransaction, output_index: u32,) -> BtcUtxoAndValue {
    info!("✔ Creating UTXO & value from tx output...");
    BtcUtxoAndValue::new(tx.output[output_index as usize].value, &create_unsigned_utxo_from_tx_output(tx, output_index))
}

fn create_unsigned_utxo_from_tx_output(tx: &BtcTransaction, output_index: u32) -> BtcUtxo {
    info!("✔ Creating unsigned UTXO from tx output...");
    let outpoint = BtcOutPoint {
        txid: tx.txid(),
        vout: output_index,
    };
    BtcUtxo {
        witness: vec![], // TODO: We don't currently support segwit txs.
        previous_output: outpoint,
        sequence: DEFAULT_BTC_SEQUENCE,
        script_sig: tx.output[output_index as usize].script_pubkey.clone(),
    }
}

fn create_unsigned_utxos_from_tx_outputs(tx: &[BtcTransaction], utxos_info: &[UtxoInfo]) -> Result<BtcUtxosAndValues> {
    Ok(
        BtcUtxosAndValues::from_vec(
            tx
                .iter()
                .enumerate()
                .map(|(i, tx)| create_btc_utxo_and_value_from_tx_output(tx, utxos_info[i].vout))
                .collect()
            )
        )
}

pub fn extract_utxos_and_add_to_state(state: State) -> Result<State> {
    info!("✔ Maybe extracting UTXOs and adding to state...");
    create_unsigned_utxos_from_tx_outputs(state.get_btc_txs()?, state.get_utxos_info()?)
        .and_then(|utxos| {
            info!(
                "✔ Total value of the {} UTXO(s): {} BTC",
                utxos.len(),
                utxos.get_total_value() as f64 / ONE_BTC,
            );
            state.add_btc_utxos_and_values(utxos)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::consensus::encode::serialize as btc_serialize;
    use crate::lib::test_utils::{
        get_sample_tx,
        SAMPLE_UTXO_INDEX,
        SAMPLE_TESTNET_TX_ID,
    };


    #[test]
    fn should_create_btc_utxo_and_value_from_tx_output() {
        let expected_value = 1666;
        let expected_serialized_utxo = "2429c0429f37f6cb8781cec1b2a474da15a2ceb1210a3d834a40daa3f4faf885000000001976a91454102783c8640c5144d039cea53eb7dbb470081488acffffffff";
        let result = create_btc_utxo_and_value_from_tx_output(
            &get_sample_tx(),
            SAMPLE_UTXO_INDEX,
        );
        assert!(result.value ==  expected_value);
        assert_eq!(result.get_serialized_utxo_hex(), expected_serialized_utxo);
    }

    #[test]
    fn should_create_unsigned_utxo_from_tx_output() {
        let expected_serialized_script = "1976a91454102783c8640c5144d039cea53eb7dbb470081488ac";
        let utxo = create_unsigned_utxo_from_tx_output(&get_sample_tx(), SAMPLE_UTXO_INDEX);
        let serialized_script = hex::encode(btc_serialize(&utxo.script_sig));
        assert!(utxo.previous_output.txid.to_string() == SAMPLE_TESTNET_TX_ID);
        assert!(serialized_script == expected_serialized_script);
    }
}
