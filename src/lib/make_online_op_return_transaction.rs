use crate::lib::{
    state::State,
    types::Result,
    get_cli_args::CliArgs,
    utils::serialize_tx_in_state,
    save_output::maybe_save_output,
    get_utxos_info::get_utxos_info_and_add_to_state,
    create_op_return_tx::create_op_return_tx_and_add_to_state,
    get_btc_private_key::get_btc_private_key_and_add_to_state,
    extract_utxos_from_utxo_info::extract_utxos_and_add_to_state,
    get_btc_txs_from_utxos_info::get_txs_from_utxo_infos_and_put_in_state,
};

pub fn make_online_op_return_transaction(cli_args: CliArgs) -> Result<String> {
    info!("✔ Making online `OP_RETURN` transaction...");
    State::init_from_cli_args(cli_args.clone())
        .and_then(get_btc_private_key_and_add_to_state)
        .and_then(get_utxos_info_and_add_to_state)
        .and_then(get_txs_from_utxo_infos_and_put_in_state)
        .and_then(extract_utxos_and_add_to_state)
        .and_then(create_op_return_tx_and_add_to_state)
        .and_then(serialize_tx_in_state)
        .and_then(|output| maybe_save_output(output, &cli_args.flag_outputPath))

}
