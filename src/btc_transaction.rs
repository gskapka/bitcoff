use crate::{
    errors::AppError,
    types::BtcUtxoAndValue,
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
    utxos_and_values: Vec<BtcUtxoAndValue>,
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
            .iter()
            .map(|utxo_and_value| utxo_and_value.get_utxo())
            .collect::<Result<Vec<BtcUtxo>>>()?,
    };
    let signatures = utxos_and_values
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
