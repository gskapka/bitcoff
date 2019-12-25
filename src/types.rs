use std::result;
use crate::errors::AppError;

pub type Result<T> = result::Result<T, AppError>;
