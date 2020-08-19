use bitcoin::{
    hashes::sha256d,
    network::constants::Network as BtcNetwork,
    blockdata::{
        script::Script as BtcScript,
        transaction::Transaction as BtcTransaction,
    },
};
use crate::lib::{
    errors::AppError,
    btc_private_key::BtcPrivateKey,
    types::{
        Bytes,
        Result,
        UtxosInfo,
        BtcTransactions,
        BtcUtxosAndValues,
        BtcAddressesAndAmounts,
    },
    get_cli_args::{
        CliArgs,
        get_network_from_cli_arg,
        get_api_endpoint_from_cli_args,
        get_addresses_and_amounts_from_cli_args,
    },
};

pub struct State {
    pub cli_args: CliArgs,
    pub network: BtcNetwork,
    pub api_endpoint: String,
    pub btc_script: Option<BtcScript>,
    pub utxos_info: Option<UtxosInfo>,
    pub btc_tx: Option<BtcTransaction>,
    pub eth_address_bytes: Option<Bytes>,
    pub utxo_json_string: Option<String>,
    pub btc_txs: Option<BtcTransactions>,
    pub btc_private_key: Option<BtcPrivateKey>,
    pub addresses_and_amounts: BtcAddressesAndAmounts,
    pub btc_utxos_and_values: Option<BtcUtxosAndValues>,
    pub eth_address_and_nonce_hash: Option<sha256d::Hash>,
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
                btc_tx: None,
                btc_txs: None,
                utxos_info: None,
                btc_script: None,
                btc_private_key: None,
                utxo_json_string: None,
                eth_address_bytes: None,
                btc_utxos_and_values: None,
                eth_address_and_nonce_hash: None,
                network: get_network_from_cli_arg(&cli_args.flag_network),
                api_endpoint: get_api_endpoint_from_cli_args(&cli_args.flag_network),
                addresses_and_amounts: get_addresses_and_amounts_from_cli_args(&cli_args.arg_to, &cli_args.arg_amount),
                cli_args,
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

    pub fn add_btc_script(
        mut self,
        btc_script: BtcScript,
    ) -> Result<State> {
        match self.btc_script {
            Some(_) => Err(AppError::Custom(
                get_no_overwrite_state_err("btc_script"))
            ),
            None => {
                self.btc_script = Some(btc_script);
                Ok(self)
            }
        }
    }

    pub fn add_eth_address_bytes(
        mut self,
        eth_address_bytes: Bytes,
    ) -> Result<State> {
        match self.eth_address_bytes {
            Some(_) => Err(AppError::Custom(
                get_no_overwrite_state_err("eth_address_bytes"))
            ),
            None => {
                self.eth_address_bytes = Some(eth_address_bytes);
                Ok(self)
            }
        }
    }

    pub fn add_eth_address_and_nonce_hash(
        mut self,
        eth_address_and_nonce_hash: sha256d::Hash,
    ) -> Result<State> {
        match self.eth_address_and_nonce_hash {
            Some(_) => Err(AppError::Custom(
                get_no_overwrite_state_err("eth_address_and_nonce_hash"))
            ),
            None => {
                self.eth_address_and_nonce_hash = Some(eth_address_and_nonce_hash);
                Ok(self)
            }
        }
    }

    pub fn add_btc_tx(mut self, btc_tx: BtcTransaction) -> Result<State> {
        match self.btc_tx {
            Some(_) => Err(AppError::Custom(
                get_no_overwrite_state_err("btc_tx"))
            ),
            None => {
                self.btc_tx = Some(btc_tx);
                Ok(self)
            }
        }
    }

    pub fn add_utxo_json_string(
        mut self,
        utxo_json_string: String
    ) -> Result<State> {
        match self.utxo_json_string {
            Some(_) => Err(AppError::Custom(
                get_no_overwrite_state_err("utxo_json_string"))
            ),
            None => {
                self.utxo_json_string = Some(utxo_json_string);
                Ok(self)
            }
        }
    }

    pub fn add_utxos_info(mut self, utxos_info: UtxosInfo) -> Result<State> {
        match self.utxos_info {
            Some(_) => Err(AppError::Custom(
                get_no_overwrite_state_err("utxos_info"))
            ),
            None => {
                self.utxos_info = Some(utxos_info);
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

    pub fn get_btc_utxos_and_values(&self) -> Result<&BtcUtxosAndValues> {
        match &self.btc_utxos_and_values {
            Some(btc_utxos_and_values) => Ok(&btc_utxos_and_values),
            None => Err(AppError::Custom(
                get_not_in_state_err("btc_utxos_and_values"))
            )
        }
    }

    pub fn get_btc_txs(&self) -> Result<&BtcTransactions> {
        match &self.btc_txs {
            Some(btc_txs) => Ok(&btc_txs),
            None => Err(AppError::Custom(
                get_not_in_state_err("btc_txs"))
            )
        }
    }

    pub fn get_utxos_info(&self) -> Result<&UtxosInfo> {
        match &self.utxos_info {
            Some(utxos_info) => Ok(&utxos_info),
            None => Err(AppError::Custom(
                get_not_in_state_err("utxos_info"))
            )
        }
    }

    pub fn get_btc_private_key(&self) -> Result<&BtcPrivateKey> {
        match &self.btc_private_key {
            Some(btc_private_key) => Ok(&btc_private_key),
            None => Err(AppError::Custom(
                get_not_in_state_err("btc_private_key"))
            )
        }
    }

    pub fn get_btc_address(&self) -> Result<String> {
        self.get_btc_private_key()
            .map(|pk| pk.to_p2pkh_btc_address())
    }

    pub fn get_btc_tx(&self) -> Result<&BtcTransaction> {
        match &self.btc_tx {
            Some(btc_tx) => Ok(&btc_tx),
            None => Err(AppError::Custom(
                get_not_in_state_err("btc_tx"))
            )
        }
    }

    pub fn get_utxo_json_string(&self) -> Result<&String> {
        match &self.utxo_json_string {
            Some(utxo_json_string) => Ok(&utxo_json_string),
            None => Err(AppError::Custom(
                get_not_in_state_err("utxo_json_string"))
            )
        }
    }

    pub fn get_eth_address_bytes(&self) -> Result<&Bytes> {
        match &self.eth_address_bytes {
            Some(eth_address_bytes) => Ok(&eth_address_bytes),
            None => Err(AppError::Custom(
                get_not_in_state_err("eth_address_bytes"))
            )
        }
    }

    pub fn get_eth_address_and_nonce_hash(&self) -> Result<&sha256d::Hash> {
        match &self.eth_address_and_nonce_hash {
            Some(eth_address_and_nonce_hash) => Ok(&eth_address_and_nonce_hash),
            None => Err(AppError::Custom(
                get_not_in_state_err("eth_address_and_nonce_hash"))
            )
        }
    }

    pub fn get_btc_script(&self) -> Result<&BtcScript> {
        match &self.btc_script {
            Some(btc_script) => Ok(&btc_script),
            None => Err(AppError::Custom(
                get_not_in_state_err("btc_script"))
            )
        }
    }
}
