use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Custom(String),
    IOError(std::io::Error),
    HexError(hex::FromHexError),
    SecpError(secp256k1::Error),
    NoneError(std::option::NoneError),
    SerdeJsonError(serde_json::Error),
    FromUtf8Error(std::str::Utf8Error),
    SystemTimeError(std::time::SystemTimeError),
    BitcoinError(bitcoin::consensus::encode::Error),
    BitcoinAddressError(bitcoin::util::address::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            AppError::Custom(ref msg) => msg.to_string(),
            AppError::IOError(ref e) =>
                format!("✘ I/O Error!\n✘ {}", e),
            AppError::HexError(ref e) =>
                format!("✘ Hex Error!\n✘ {}", e),
            AppError::Base58Error(ref e) =>
                format!("✘ Base58 Error!\n✘ {}", e),
            AppError::BitcoinError(ref e) =>
                format!("✘ Bitcoin Error!\n✘ {}", e),
            AppError::SerdeJsonError(ref e) =>
                format!("✘ Serde-Json Error!\n✘ {}", e),
            AppError::SystemTimeError(ref e) =>
                format!("✘ System time error!\n✘ {}", e),
            AppError::FromUtf8Error(ref e) =>
                format!("✘ From utf8 error: \n✘ {:?}", e),
            AppError::SecpError(ref e) =>
                format!("✘ secp256k1 error: \n✘ {:?}", e),
            AppError::NoneError(ref e) =>
                format!("✘ Nothing to unwrap!\n✘ {:?}", e),
            AppError::BitcoinAddressError(ref e) =>
                format!("✘ Bitcoin Address Error!\n✘ {}", e),
        };
        f.write_fmt(format_args!("{}", msg))
    }
}

impl From<std::str::Utf8Error> for AppError {
    fn from(e: std::str::Utf8Error) -> AppError {
        AppError::FromUtf8Error(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> AppError {
        AppError::IOError(e)
    }
}

impl From<bitcoin::consensus::encode::Error> for AppError {
    fn from(e: bitcoin::consensus::encode::Error) -> AppError {
        AppError::BitcoinError(e)
    }
}

impl From<secp256k1::Error> for AppError {
    fn from(e: secp256k1::Error) -> AppError {
        AppError::SecpError(e)
    }
}

impl From<hex::FromHexError> for AppError {
    fn from(e: hex::FromHexError) -> AppError {
        AppError::HexError(e)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> AppError {
        AppError::SerdeJsonError(e)
    }
}

impl From<bitcoin::util::address::Error> for AppError {
    fn from(e: bitcoin::util::address::Error) -> AppError {
        AppError::BitcoinAddressError(e)
    }
}

impl From<std::time::SystemTimeError> for AppError {
    fn from(e: std::time::SystemTimeError) -> AppError {
        AppError::SystemTimeError(e)
    }
}

impl From<std::option::NoneError> for AppError {
    fn from(e: std::option::NoneError) -> AppError {
        AppError::NoneError(e)
    }
}
