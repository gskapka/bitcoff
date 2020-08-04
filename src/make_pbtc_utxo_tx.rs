use bitcoin::{
    consensus::encode::serialize as btc_serialize,
    blockdata::{
        transaction::{
            TxIn as BtcUtxo,
            TxOut as BtcTxOut,
            Transaction as BtcTransaction,
        },
        script::{
            Script as BtcScript,
            Builder as BtcScriptBuilder,
        },
    },
};
use crate::{
    state::State,
    errors::AppError,
    get_cli_args::{
        CliArgs,
        get_nonce_from_cli_arg,
    },
    btc_private_key::BtcPrivateKey,
    get_btc_private_key::get_btc_private_key_and_add_to_state,
    utxo_codec::get_utxos_from_utxo_json_string_and_add_to_state,
    get_utxo_json_string::get_utxo_json_string_from_cli_args_and_add_to_state,
    types::{
        Bytes,
        Result,
        BtcUtxoAndValue,
    },
    utils::{
        calculate_btc_tx_fee,
        create_new_tx_output,
        get_total_value_of_utxos_and_values,
        get_change_address_from_cli_args_in_state,
    },
    get_pbtc_deposit_address::{
        generate_pbtc_script_sig,
        get_eth_address_and_nonce_hash,
        get_eth_address_from_cli_args_and_put_in_state,
    },
};

pub fn get_pbtc_script_sig<'a>(signature_slice: &'a[u8], redeem_script: &BtcScript) -> BtcScript {
    BtcScriptBuilder::new()
        .push_slice(&signature_slice)
        .push_slice(redeem_script.as_bytes())
        .into_script()
}

fn get_btc_script_and_put_in_state(state: State) -> Result<State> {
    info!("✔ Getting BTC redeem script and putting in state...");
    generate_pbtc_script_sig( // TODO Rename to "redeem script" or something?
        &state.get_btc_private_key()?.to_public_key_slice(),
        state.get_eth_address_and_nonce_hash()?
    )
        .and_then(|script| state.add_btc_script(script))
}

pub fn get_eth_address_and_nonce_hash_and_put_in_state(state: State) -> Result<State> {
    get_eth_address_and_nonce_hash(
        state.get_eth_address_bytes()?,
        // FIXME: Only using the first things we've passed in!
        &get_nonce_from_cli_arg(&state.cli_args.arg_ethAddressNonce[0])?,
    )
        .and_then(|hash| state.add_eth_address_and_nonce_hash(hash))
}

fn make_pbtc_tx_and_put_in_state(
    state: State
) -> Result<State> {
    info!("✔ Making pBTC tx and putting in state...");
    create_signed_raw_btc_tx_for_n_input_n_outputs(
        state.cli_args.flag_fee,
        state.addresses_and_amounts.clone(),
        &get_change_address_from_cli_args_in_state(&state)?,
        *state.get_btc_private_key()?,
        state.get_btc_utxos_and_values()?.clone(),
        None,
        state.get_btc_script()?,
    )
        .and_then(|tx| state.add_btc_tx(tx))
}

pub fn make_pbtc_utxo_tx(cli_args: CliArgs) -> Result<String> {
    info!("✔ Spending pBTC UTXO(s)...");
    State::init_from_cli_args(cli_args)
        .and_then(get_btc_private_key_and_add_to_state)
        .and_then(get_utxo_json_string_from_cli_args_and_add_to_state)
        .and_then(get_utxos_from_utxo_json_string_and_add_to_state)
        .and_then(get_eth_address_from_cli_args_and_put_in_state)
        .and_then(get_eth_address_and_nonce_hash_and_put_in_state)
        .and_then(get_btc_script_and_put_in_state)
        .and_then(make_pbtc_tx_and_put_in_state)
        .and_then(|state| Ok(hex::encode(btc_serialize(state.get_btc_tx()?))))
}

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
    redeem_script: &BtcScript,
) -> Result<BtcTransaction> {
    debug!("Redeem script: {}", redeem_script);
    debug!("Redeem script serialized: {}", hex::encode(redeem_script.as_bytes()));
    let total_to_spend: u64 = recipient_addresses_and_amounts.iter().map(|(_, amount)| amount).sum();
    let fee = calculate_btc_tx_fee(
        utxos_and_values.len(),
        match &maybe_op_return_output {
            None => recipient_addresses_and_amounts.len(),
            Some(_) => recipient_addresses_and_amounts.len() + 1,
        },
        sats_per_byte,
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
        outputs.push(create_new_tx_output(&change, remainder_btc_address)?)
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
        .map(|(i, _)| tx.signature_hash(i, &redeem_script, SIGN_ALL_HASH_TYPE as u32))
        .map(|hash| hash.to_vec())
        .map(|tx_hash_to_sign|
            btc_private_key
                .sign_hash_and_append_btc_hash_type(tx_hash_to_sign.to_vec(), SIGN_ALL_HASH_TYPE as u8)
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
                script_sig: get_pbtc_script_sig(&signatures[i], &redeem_script),
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
