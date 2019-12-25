#![feature(try_trait)]

pub mod state;
pub mod types;
pub mod utils;
pub mod errors;
pub mod test_utils;
pub mod usage_info;
pub mod get_cli_args;
pub mod make_transaction;
pub mod initialize_logger;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

use crate::{
    types::Result,
    errors::AppError,
    usage_info::USAGE_INFO,
    make_transaction::make_transaction,
    initialize_logger::initialize_logger,
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
                CliArgs {cmd_makeTx: true, ..} => {
                    info!("✔ Making transaction...");
                    make_transaction(cli_args)
                }
                _ => Err(AppError::Custom(USAGE_INFO.to_string()))
            }
        ) {
            Ok(json_string) => {
                trace!("{}", json_string);
                Ok(())
            },
            Err(e) => {
                error!("{}", e);
                std::process::exit(1);
            }
        }
}
