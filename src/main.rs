#![feature(try_trait)]

pub mod state;
pub mod types;
pub mod utils;
pub mod base58;
pub mod errors;
pub mod get_utxos;
pub mod constants;
pub mod test_utils;
pub mod usage_info;
pub mod get_cli_args;
pub mod get_utxos_info;
pub mod btc_private_key;
pub mod initialize_logger;
pub mod get_btc_private_key;
pub mod get_btc_txs_from_tx_ids;
pub mod make_op_return_transaction;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

use crate::{
    types::Result,
    errors::AppError,
    usage_info::USAGE_INFO,
    initialize_logger::initialize_logger,
    make_op_return_transaction::make_op_return_transaction,
    get_cli_args::{
        CliArgs,
        get_cli_args,
    },
};

fn main() -> Result<()> {
    match initialize_logger()
        .and_then(|_| get_cli_args())
        .and_then(|cli_args|
            match cli_args {
                CliArgs {cmd_makeOpReturnTx: true, ..} => 
                    make_op_return_transaction(cli_args),
                _ => Err(AppError::Custom(USAGE_INFO.to_string()))
            }
        ) {
            Ok(json_string) => {
                info!("{}", json_string);
                Ok(())
            },
            Err(e) => {
                error!("{}", e);
                std::process::exit(1);
            }
        }
}
