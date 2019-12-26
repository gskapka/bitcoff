use log;
use simplelog::*;
use log::LevelFilter;
use crate::{
    types::Result,
    errors::AppError,
};

pub fn initialize_logger() -> Result<()> {
    match TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
    ) {
        Ok(_) => {
            info!("✔ Logger initialized successfully!");
            Ok(())
        },
        Err(e) => Err(AppError::Custom(e.to_string()))
    }
}
