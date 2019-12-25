use docopt::Docopt;
use bitcoin::network::constants::Network as BtcNetwork;
use crate::{
    types::Result,
    errors::AppError,
    usage_info::USAGE_INFO,
};

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CliArgs {
    pub cmd_makeTx: bool,
    pub flag_network: String,
    pub flag_keyfile: String,
    pub arg_utxo_index: usize,
    pub arg_btc_block_hash: String,
}

pub fn get_network_from_cli_arg(network_cli_arg: &String) -> BtcNetwork {
    info!("✔ Getting network from cli-arg: '{}'", network_cli_arg);
    match &network_cli_arg[..] {
        "Testnet" => {
            info!("✔ Using network: 'Testnet'");
            BtcNetwork::Testnet
        }
        _ => {
            info!("✔ Using network: 'Bitcoin'");
            BtcNetwork::Bitcoin
        }
    }
}


pub fn get_cli_args() -> Result<CliArgs> {
    info!("✔ Getting CLI args...");
    match Docopt::new(USAGE_INFO) 
        .and_then(|d| d.deserialize()) {
            Ok(cli_args) => Ok(cli_args),
            Err(e) => Err(AppError::Custom(e.to_string()))
        }
}
