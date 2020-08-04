use crate::{
    state::State,
    types::Result,
    get_cli_args::CliArgs,
    save_output::maybe_save_output,
    utxo_codec::get_utxo_json_string_from_utxos_in_state,
    extract_utxos_from_utxo_info::extract_utxos_and_add_to_state,
    get_btc_txs_from_utxos_info::get_txs_from_utxo_infos_and_put_in_state,
    get_utxos_info::get_utxos_info_for_address_in_cli_args_and_add_to_state,
};

pub fn get_utxos_for_address(cli_args: CliArgs) -> Result<String> {
    info!("✔ Getting UTXOs for address: {}", &cli_args.arg_btcAddress);
    State::init_from_cli_args(cli_args.clone())
        .and_then(get_utxos_info_for_address_in_cli_args_and_add_to_state)
        .and_then(get_txs_from_utxo_infos_and_put_in_state)
        .and_then(extract_utxos_and_add_to_state)
        .and_then(get_utxo_json_string_from_utxos_in_state)
        .and_then(|output| maybe_save_output(output, &cli_args.flag_outputPath))
}
