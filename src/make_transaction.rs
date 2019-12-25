use crate::{
    state::State,
    types::Result,
    get_cli_args::CliArgs,
    get_btc_private_key::maybe_get_btc_private_key_and_add_to_state,
};

pub fn make_transaction(cli_args: CliArgs) -> Result<String> {
    info!("✔ Creating BTC transaction...");
    State::init_from_cli_args(cli_args)
        .and_then(maybe_get_btc_private_key_and_add_to_state)
        .and_then(|_| Ok("✔ Done!".to_string()))
}
