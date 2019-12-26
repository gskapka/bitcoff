use docopt::Docopt;
use bitcoin::network::constants::Network as BtcNetwork;
use crate::{
    types::Result,
    errors::AppError,
    usage_info::USAGE_INFO,
    constants::BLOCK_EXPLORER_URL,
};

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CliArgs {
    pub arg_to: String, // FIXME How to check is valid address?
    pub flag_fee: usize,
    pub arg_data: String,
    pub flag_value: String,
    pub flag_change: String, // FIXME How to check is valid address?
    pub flag_network: String,
    pub flag_keyfile: String,
    pub arg_tx_id: Vec<String>,
    pub cmd_makeOpReturnTx: bool,
    pub arg_utxo_indices: Vec<u32>,
}

/* convert string to usize. Unless it's the default 'max'
pub fn get_amount_from_cli_arg() {
}
*/

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

pub fn get_api_endpoint_from_cli_args(network_cli_arg: &String) -> String {
    info!("✔ Getting API endpoint...");
    let api_url = match &network_cli_arg[..] {
        "Testnet" => format!("{}testnet/api/", BLOCK_EXPLORER_URL),
        _ => format!("{}/api/", BLOCK_EXPLORER_URL),
    };
    info!("✔ Using API endpoint: `{}`", api_url);
    api_url
}

pub fn get_cli_args() -> Result<CliArgs> {
    info!("✔ Getting CLI args...");
    match Docopt::new(USAGE_INFO) 
        .and_then(|d| d.deserialize()) {
            Ok(cli_args) => Ok(cli_args),
            Err(e) => Err(AppError::Custom(e.to_string()))
        }
}
