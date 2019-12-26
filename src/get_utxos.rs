use bitcoin::{
    blockdata::{
        script::Script as BtcScript,
        transaction::{
            TxIn as BtcUtxo,
            TxOut as BtcTxOut,
            OutPoint as BtcOutPoint,
            Transaction as BtcTransaction,
        },
    },
};
use crate::{
    state::State,
    constants::DEFAULT_BTC_SEQUENCE,
    types::{
        Result,
        UtxosInfo,
        BtcUtxoAndValue,
        BtcUtxosAndValues,
    },
    utils::{
        serialize_btc_utxo,
        deserialize_btc_utxo,
    },
};

pub fn create_btc_utxo_and_value_from_tx_output(
    tx: &BtcTransaction,
    output_index: u32,
) -> BtcUtxoAndValue {
    info!("✔ Creating UTXO & value from tx output...");
    BtcUtxoAndValue::new(
        tx.output[output_index as usize].value,
        &create_unsigned_utxo_from_tx_output(tx, output_index)
    )
}

fn create_unsigned_utxo_from_tx_output( 
    tx: &BtcTransaction,
    output_index: u32,
) -> BtcUtxo {
    info!("✔ Creating unsigned UTXO from tx output...");
    let outpoint = BtcOutPoint {
        txid: tx.txid(),
        vout: output_index,
    };
    BtcUtxo {
        witness: vec![], // TODO: We don't currently support segwit txs.
        previous_output: outpoint,
        sequence: DEFAULT_BTC_SEQUENCE,
        script_sig: tx
            .output[output_index as usize]
            .script_pubkey
            .clone(),
    }
}

fn create_unsigned_utxos_from_tx_outputs(
    tx: &Vec<BtcTransaction>,
    utxos_info: &UtxosInfo,
) -> Result<BtcUtxosAndValues> {
    Ok(
        tx
            .iter()
            .enumerate()
            .map(|(i, tx)| 
                create_btc_utxo_and_value_from_tx_output(
                     tx, 
                     utxos_info[i].vout,
                 )
             )
            .collect::<BtcUtxosAndValues>()
    )
}

pub fn extract_utxos_and_add_to_state(state: State) -> Result<State> {
    info!("✔ Maybe extracting UTXOs and adding to state...");
    create_unsigned_utxos_from_tx_outputs(
        state.get_btc_txs()?,
        state.get_utxos_info()?,
    )
        .and_then(|utxos| {
            debug!("✔ The extracted UTXOs: {:?}", utxos);
            state.add_btc_utxos_and_values(utxos)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::consensus::encode::serialize as btc_serialize;
    use crate::test_utils::{
        get_sample_tx,
        SAMPLE_UTXO_INDEX,
        get_sample_tx_output,
        SAMPLE_TESTNET_TX_ID,
    };


    #[test]
    fn should_create_btc_utxo_and_value_from_tx_output() {
        let expected_value = 1666;
        let expected_serialized_utxo = vec![
            36, 41, 192, 66, 159, 55, 246, 203, 135, 129, 206, 193, 
            178, 164, 116, 218, 21, 162, 206, 177, 33, 10, 61, 131, 
            74, 64, 218, 163, 244, 250, 248, 133, 0, 0, 0, 0, 25, 
            118, 169, 20, 84, 16, 39, 131, 200, 100, 12, 81, 68, 208, 
            57, 206, 165, 62, 183, 219, 180, 112, 8, 20, 136, 172, 
            255, 255, 255, 255
        ];
        let result = create_btc_utxo_and_value_from_tx_output(
            &get_sample_tx(),
            SAMPLE_UTXO_INDEX,
        );
        assert!(result.value ==  expected_value);
        assert!(result.serialized_utxo == expected_serialized_utxo);
    }

    #[test]
    fn should_create_unsigned_utxo_from_tx_output() {
        let expected_serialized_script = 
            "1976a91454102783c8640c5144d039cea53eb7dbb470081488ac"; 
        let utxo = create_unsigned_utxo_from_tx_output(
            &get_sample_tx(),
            SAMPLE_UTXO_INDEX,
        );
        let serialized_script = hex::encode(btc_serialize(&utxo.script_sig));
        assert!(utxo.previous_output.txid.to_string() == SAMPLE_TESTNET_TX_ID);
        assert!(serialized_script == expected_serialized_script);
    }
}
