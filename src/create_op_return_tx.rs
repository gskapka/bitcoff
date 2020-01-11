use crate::{
    state::State,
    types::Result,
    utils::{
        get_op_return_output,
        get_change_address_from_cli_args_in_state,
    }, 
    btc_transaction::create_signed_raw_btc_tx_for_n_input_n_outputs,
};

pub fn create_op_return_tx_and_add_to_state(state: State) -> Result<State> {
    info!("✔ Creating `OP_RETURN` tx and adding to state...");
    create_signed_raw_btc_tx_for_n_input_n_outputs(
        state.cli_args.flag_fee.clone(),
        state.addresses_and_amounts.clone(),
        &get_change_address_from_cli_args_in_state(&state)?,
        state.get_btc_private_key()?.clone(),
        state.get_btc_utxos_and_values()?.clone(),
        Some(
            get_op_return_output(
                &hex::decode(state.cli_args.arg_data.clone())?
            )?
        ),
    )
        .and_then(|tx| state.add_btc_tx(tx))
}
