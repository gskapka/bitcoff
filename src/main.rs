#![feature(try_trait)]
#![allow(clippy::match_bool)]

mod lib;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

use crate::lib::{
    types::Result,
    errors::AppError,
    get_utxos::get_utxos,
    usage_info::USAGE_INFO,
    get_utxos_for_address::get_utxos_for_address,
    make_online_transaction::make_online_transaction,
    make_offline_transaction::make_offline_transaction,
    initialize_logger::maybe_initialize_logger_and_return_cli_args,
    make_online_op_return_transaction::make_online_op_return_transaction,
    make_offline_op_return_transaction::make_offline_op_return_transaction,
    get_cli_args::{
        CliArgs,
        get_cli_args,
    },
};

fn main() -> Result<()> {
    match get_cli_args()
        .and_then(maybe_initialize_logger_and_return_cli_args)
        .and_then(|cli_args|
            match cli_args {
                CliArgs {cmd_getUtxos: true, ..} => get_utxos(cli_args),
                CliArgs {cmd_makeOnlineTx: true, ..} => make_online_transaction(cli_args),
                CliArgs {cmd_makeOfflineTx: true, ..} => make_offline_transaction(cli_args),
                CliArgs {cmd_getUtxosForAddress: true, ..} => get_utxos_for_address(cli_args),
                CliArgs {cmd_makeOnlineOpReturnTx: true, ..} => make_online_op_return_transaction(cli_args),
                CliArgs {cmd_makeOfflineOpReturnTx: true, ..} => make_offline_op_return_transaction(cli_args),
                _ => Err(AppError::Custom(USAGE_INFO.to_string()))
            }
        ) {
            Ok(json_string) => {
                println!("{}", json_string);
                Ok(())
            },
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            }
        }
}
