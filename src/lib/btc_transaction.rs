use crate::lib::{
    errors::AppError,
    btc_private_key::BtcPrivateKey,
    utils::{
        get_script_sig,
        create_new_tx_output,
        calculate_btc_tx_fee,
        get_total_value_of_utxos_and_values,
    },
    types::{
        Bytes,
        Result,
        BtcUtxosAndValues,
    },
};
use bitcoin::{
    blockdata::{
        transaction::{
            TxIn as BtcUtxo,
            TxOut as BtcTxOut,
            Transaction as BtcTransaction,
        },
    }
};

// NOTE: Current tx constants. Could make generic in future if needed.
pub const VERSION: u32 = 1;
pub const LOCK_TIME: u32 = 0;
pub const SIGN_ALL_HASH_TYPE: u8 = 1;
pub const UTXO_VALUE_TOO_LOW_ERROR: &str = "✘ Not enough UTXO value to make transaction!";

pub fn create_signed_raw_btc_tx_for_n_input_n_outputs(
    sats_per_byte: usize,
    recipient_addresses_and_amounts: Vec<(String, u64)>, // TODO MAKE A TYPE?
    remainder_btc_address: &str,
    btc_private_key: BtcPrivateKey,
    utxos_and_values: &BtcUtxosAndValues,
    maybe_op_return_output: Option<BtcTxOut>,
) -> Result<BtcTransaction> {
    let total_to_spend: u64 = recipient_addresses_and_amounts
        .iter()
        .map(|(_, amount)| amount)
        .sum();

    let fee = calculate_btc_tx_fee(
        utxos_and_values.len(),
        match &maybe_op_return_output {
            None => recipient_addresses_and_amounts.len(),
            Some(_) => recipient_addresses_and_amounts.len() + 1,
        },
        sats_per_byte
    );
    let utxo_total = get_total_value_of_utxos_and_values(&utxos_and_values);
    info!("✔ UTXO(s) total:  {}", utxo_total);
    info!("✔ Outgoing total: {}", total_to_spend);
    info!("✔ Change amount:  {}", utxo_total - (total_to_spend + fee));
    info!("✔ Tx fee:         {}", fee);
    if total_to_spend + fee > utxo_total {
        return Err(AppError::Custom(UTXO_VALUE_TOO_LOW_ERROR.to_string()))
    }
    let mut outputs = recipient_addresses_and_amounts
        .iter()
        .map(|(address, amount)| create_new_tx_output(&amount, address))
        .collect::<Result<Vec<BtcTxOut>>>()?;
    if let Some(op_return_output) = maybe_op_return_output {
        outputs.push(op_return_output);
    };
    let change = utxo_total - total_to_spend - fee;
    if change > 0 {
        outputs.push(
            create_new_tx_output(&change, remainder_btc_address)?
        )
    };
    let tx = BtcTransaction {
        output: outputs,
        version: VERSION,
        lock_time: LOCK_TIME,
        input: utxos_and_values
            .to_vec()
            .iter()
            .map(|utxo_and_value| utxo_and_value.get_utxo())
            .collect::<Result<Vec<BtcUtxo>>>()?,
    };
    let signatures = utxos_and_values
        .to_vec()
        .iter()
        .map(|utxo_and_value| utxo_and_value.get_utxo())
        .collect::<Result<Vec<BtcUtxo>>>()?
        .iter()
        .enumerate()
        .map(|(i, utxo)|
            tx.signature_hash(
                i,
                &utxo.script_sig,
                SIGN_ALL_HASH_TYPE as u32
            )
        )
        .map(|hash| hash.to_vec())
        .map(|tx_hash_to_sign|
            btc_private_key
                .sign_hash_and_append_btc_hash_type(
                    tx_hash_to_sign.to_vec(),
                    SIGN_ALL_HASH_TYPE as u8,
                )
        )
        .collect::<Result<Vec<Bytes>>>()?;
    let utxos_with_signatures = utxos_and_values
        .to_vec()
        .iter()
        .map(|utxo_and_value| utxo_and_value.get_utxo())
        .collect::<Result<Vec<BtcUtxo>>>()?
        .iter()
        .enumerate()
        .map(|(i, utxo)|
            BtcUtxo {
                sequence: utxo.sequence,
                witness: utxo.witness.clone(),
                previous_output: utxo.previous_output,
                script_sig: get_script_sig(
                    &signatures[i],
                    &btc_private_key.to_public_key_slice(),
                ),
            }
         )
        .collect::<Vec<BtcUtxo>>();
    Ok(
        BtcTransaction {
            output: tx.output,
            version: tx.version,
            lock_time: tx.lock_time,
            input: utxos_with_signatures,
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::consensus::encode::serialize as btc_serialize;
    use crate::lib::test_utils::{
        get_sample_utxo,
        get_sample_btc_private_key,
    };

    #[test]
    fn should_create_tx_correctly() {
        let expected_result = "01000000016e3fa15afcd9b579b7ed082e0ee8cfba1f27a6cf007cb7ca95b06ab0fda2880c020000006b483045022100d5dec195ae624af5708ca7834b9e052b93bcf935c92c19dcf1be39ce1aa60d31022019db89b0938e14c82f1f66ad8ab7377fe5895c78293df5b3a50ffa221fa04700012103d2a5e3b162eb580fe2ce023cd5e0dddbb6286923acde77e3e5468314dc9373f7ffffffff0289130000000000001976a9149ae6e42c56f1ea319cfc704ad50db0683015029b88ac333a0d00000000001976a91454102783c8640c5144d039cea53eb7dbb470081488ac00000000";
        let sats_per_byte = 100;
        let recipients_and_amounts = vec![("mudzxCq9aCQ4Una9MmayvJVCF1Tj9fypiM".to_string(), 5001)];
        let remainder_btc_address = "moBSQbHn7N9BC9pdtAMnA7GBiALzNMQJyE";
        let btc_private_key = get_sample_btc_private_key();
        let utxos_and_values = BtcUtxosAndValues::from_vec(vec![get_sample_utxo()]);
        let maybe_op_return_output = None;
        let result = create_signed_raw_btc_tx_for_n_input_n_outputs(
            sats_per_byte,
            recipients_and_amounts,
            remainder_btc_address,
            btc_private_key,
            &utxos_and_values,
            maybe_op_return_output,
        ).unwrap();
        let result_hex = hex::encode(btc_serialize(&result));
        assert_eq!(result_hex, expected_result);
    }
}
