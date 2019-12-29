use bitcoin::consensus::encode::serialize as btc_serialize;
use crate::{
    state::State,
    types::Result,
    get_cli_args::CliArgs,
    utils::serialize_tx_in_state,
    save_output::maybe_save_output,
    create_tx::create_tx_and_add_to_state,
    create_op_return_tx::create_op_return_tx_and_add_to_state,
    get_btc_private_key::get_btc_private_key_and_add_to_state,
    utxo_codec::get_utxos_from_utxo_json_string_and_add_to_state,
    get_utxo_json_string::get_utxo_json_string_from_cli_args_and_add_to_state,
};

pub fn make_offline_transaction(cli_args: CliArgs) -> Result<String> {
    info!("✔ Making offline transaction...");
    State::init_from_cli_args(cli_args.clone())
        .and_then(get_utxo_json_string_from_cli_args_and_add_to_state)
        .and_then(get_btc_private_key_and_add_to_state)
        .and_then(get_utxos_from_utxo_json_string_and_add_to_state)
        .and_then(create_tx_and_add_to_state)
        .and_then(serialize_tx_in_state)
        .and_then(|output| maybe_save_output(output, &cli_args.flag_outputPath))
}
