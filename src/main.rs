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
pub mod get_utxo_json;
pub mod get_utxos_info;
pub mod btc_transaction;
pub mod btc_private_key;
pub mod initialize_logger;
pub mod create_op_return_tx;
pub mod get_btc_private_key;
pub mod get_btc_txs_from_utxos_info;
pub mod extract_utxos_from_utxo_info;
pub mod make_online_op_return_transaction;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

use crate::{
    types::Result,
    errors::AppError,
    get_utxos::get_utxos,
    usage_info::USAGE_INFO,
    initialize_logger::initialize_logger,
    make_online_op_return_transaction::make_online_op_return_transaction,
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
                CliArgs {cmd_getUtxos: true, ..} => 
                    get_utxos(cli_args),
                CliArgs {cmd_makeOnlineOpReturnTx: true, ..} => 
                    make_online_op_return_transaction(cli_args),
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
