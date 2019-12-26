use crate::{
    state::State,
    types::Result,
    get_cli_args::CliArgs,
    get_utxos::extract_utxos_and_add_to_state,
    get_utxos_info::get_utxos_info_and_add_to_state,
    get_btc_private_key::maybe_get_btc_private_key_and_add_to_state,
    get_btc_txs_from_utxos_info::get_txs_from_utxo_infos_and_put_in_state,
};

pub fn make_op_return_transaction(cli_args: CliArgs) -> Result<String> {
    info!("✔ Creating BTC transaction...");
    State::init_from_cli_args(cli_args)
        .and_then(maybe_get_btc_private_key_and_add_to_state)
        .and_then(get_utxos_info_and_add_to_state)
        .and_then(get_txs_from_utxo_infos_and_put_in_state)
        .and_then(extract_utxos_and_add_to_state)
        //.and_then(create_op_return_tx) // Check UTXO values are enough!
        .and_then(|_| Ok("✔ Done!".to_string()))
}
