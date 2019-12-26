use bitcoin::network::constants::Network as BtcNetwork;
use crate::{
    types::{
        Result,
        BtcTransactions,
        BtcUtxosAndValues,
    },
    errors::AppError,
    get_cli_args::{
        CliArgs,
        get_network_from_cli_arg,
        get_api_endpoint_from_cli_args,
    },
    btc_private_key::BtcPrivateKey,
};

pub struct State {
    pub cli_args: CliArgs,
    pub network: BtcNetwork,
    pub api_endpoint: String,
    pub btc_txs: Option<BtcTransactions>,
    pub btc_private_key: Option<BtcPrivateKey>,
    pub btc_utxos_and_values: Option<BtcUtxosAndValues>, 
}

pub fn get_no_overwrite_state_err(substring: &str) -> String {
    format!("✘ Cannot overwrite {} in state!" , substring)
}

pub fn get_not_in_state_err(substring: &str) -> String {
    format!("✘ No {} in state!" , substring)
}

impl State {
    pub fn init_from_cli_args(
        cli_args: CliArgs
    ) -> Result<State> {
        Ok(
            State {
                network: 
                    get_network_from_cli_arg(&cli_args.flag_network),
                api_endpoint:  
                    get_api_endpoint_from_cli_args(&cli_args.flag_network),
                cli_args,
                btc_txs: None,
                btc_private_key: None,
                btc_utxos_and_values: None,
            }
        )
    }

    pub fn add_btc_private_key( 
        mut self,
        btc_private_key: BtcPrivateKey,
    ) -> Result<State> {
        match self.btc_private_key {
            Some(_) => Err(AppError::Custom(
                get_no_overwrite_state_err("btc_private_key"))
            ),
            None => {
                self.btc_private_key = Some(btc_private_key);
                Ok(self)
            }
        }
    }

    pub fn add_btc_txs( 
        mut self,
        btc_txs: BtcTransactions,
    ) -> Result<State> {
        match self.btc_txs {
            Some(_) => Err(AppError::Custom(
                get_no_overwrite_state_err("btc_txs"))
            ),
            None => {
                self.btc_txs = Some(btc_txs);
                Ok(self)
            }
        }
    }

    pub fn add_btc_utxos_and_values( 
        mut self,
        btc_utxos_and_values: BtcUtxosAndValues,
    ) -> Result<State> {
        match self.btc_utxos_and_values {
            Some(_) => Err(AppError::Custom(
                get_no_overwrite_state_err("btc_utxos_and_values"))
            ),
            None => {
                self.btc_utxos_and_values = Some(btc_utxos_and_values);
                Ok(self)
            }
        }
    }

    pub fn get_btc_utxos_and_values( 
        &self
    ) -> Result<&BtcUtxosAndValues> {
        match &self.btc_utxos_and_values {
            Some(btc_utxos_and_values) => Ok(&btc_utxos_and_values),
            None => Err(AppError::Custom(
                get_not_in_state_err("btc_utxos_and_values"))
            )
        }
    }

    pub fn get_btc_txs( 
        &self
    ) -> Result<&BtcTransactions> {
        match &self.btc_txs {
            Some(btc_txs) => Ok(&btc_txs),
            None => Err(AppError::Custom(
                get_not_in_state_err("btc_txs"))
            )
        }
    }
}
