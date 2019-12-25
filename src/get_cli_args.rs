use docopt::Docopt;
use std::{
    path::Path,
    fs::read_to_string,
};
use crate::{
    types::Result,
    errors::AppError,
    usage_info::USAGE_INFO,
};

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CliArgs {
    pub cmd_makeTx: bool,
    pub arg_utxo_index: usize,
    pub arg_btc_block_hash: String,
}

pub fn get_cli_args() -> Result<CliArgs> {
    trace!("✔ Getting CLI args...");
    match Docopt::new(USAGE_INFO) 
        .and_then(|d| d.deserialize()) {
            Ok(cli_args) => Ok(cli_args),
            Err(e) => Err(AppError::Custom(e.to_string()))
        }
}
