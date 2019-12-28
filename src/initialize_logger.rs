use log;
use simplelog::*;
use log::LevelFilter;
use crate::{
    types::Result,
    errors::AppError,
    get_cli_args::CliArgs,
};

pub fn maybe_initialize_logger_and_return_cli_args(
    cli_args: CliArgs
) -> Result<CliArgs> {
    match &cli_args.flag_logLevel[..] {
        "none" => Ok(cli_args),
        _ => match TermLogger::init(
            match &cli_args.flag_logLevel[..] {
                "info" => LevelFilter::Info,
                "warn" => LevelFilter::Warn,
                "debug" => LevelFilter::Debug,
                "error" => LevelFilter::Error,
                "trace" => LevelFilter::Trace,
                _ => LevelFilter::Trace,
            },
            Config::default(),
            TerminalMode::Mixed,
        ) {
            Ok(_) => {
                info!("✔ Logger initialized successfully!");
                Ok(cli_args)
            },
            Err(e) => Err(AppError::Custom(e.to_string()))
        }
    }
}
