use std::result;
use crate::errors::AppError;
use bitcoin::blockdata::transaction::Transaction as BtcTransaction;

pub type Bytes = Vec<u8>;
pub type BtcTransactions = Vec<BtcTransaction>;
pub type Result<T> = result::Result<T, AppError>;
