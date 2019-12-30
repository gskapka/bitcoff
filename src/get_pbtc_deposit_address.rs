use secp256k1::{
    Message,
    Secp256k1,
    Signature,
    key::{
        SecretKey,
        PublicKey,
    },
};
use bitcoin::{
    hashes::{
        Hash,
        sha256d,
    },
    util::{
        key::PublicKey as BtcPublicKey,
        address::Address as BtcAddress,
    },
    network::constants::Network as BtcNetwork,
    consensus::encode::serialize as btc_serialize,
    consensus::encode::deserialize as btc_deserialize,
    blockdata::{
        opcodes,
        block::Block as BtcBlock,
        transaction::{
            TxIn as BtcUtxo,
            TxOut as BtcTxOut,
            Transaction as BtcTransaction,
        },
        script::{
            Script as BtcScript,
            Builder as BtcScriptBuilder,
        },
    },
};
use crate::{
    state::State,
    base58::from as from_base58,
    get_btc_private_key::get_btc_private_key_and_add_to_state,
    types::{
        Bytes,
        Result,
    },
    get_cli_args::{
        CliArgs,
        get_nonce_from_cli_arg,
        get_network_from_cli_arg,
    },
};

fn convert_eth_address_to_bytes(eth_address: &String) -> Result<Bytes> {
    Ok(hex::decode(&eth_address[..].replace("0x", ""))?)
}

fn get_eth_address_from_cli_args_and_put_in_state(
    state: State
) -> Result<State> {
    convert_eth_address_to_bytes(&state.cli_args.arg_ethAddress)
        .and_then(|bytes| state.add_eth_address_bytes(bytes))
}


fn get_eth_address_and_nonce_hash(
    eth_address: &Bytes,
    nonce: &u64,
) -> Result<sha256d::Hash> {
    info!(
        "✔ Getting ETH address & nonce hash from ETH address: 0x{} & nonce: {}",
        hex::encode(eth_address),
        nonce,
    );
    let mut vec = eth_address.clone();
    vec.append(&mut nonce.to_le_bytes().to_vec());
    Ok(sha256d::Hash::hash(&vec))
}

fn get_eth_address_and_nonce_hash_and_put_in_state(
    state: State
) -> Result<State> {
    get_eth_address_and_nonce_hash(
        state.get_eth_address_bytes()?, 
        &get_nonce_from_cli_arg(&state.cli_args.flag_nonce)?,
    )
        .and_then(|hash| state.add_eth_address_and_nonce_hash(hash))
}

pub fn convert_btc_address_to_pub_key_hash_bytes(
    btc_address: &str
) -> Result<Bytes> {
    Ok(from_base58(btc_address)?[1..21].to_vec())
}

fn generate_pbtc_script_sig<'a>(
    recipient: &str,
    utxo_spender_pub_key_slice: &'a[u8],
    eth_address_and_nonce_hash: &sha256d::Hash,
) -> Result<BtcScript> {
    info!(
        "✔ Generating pBTC `script_sig` for recipient: {}",
        recipient,
    );
    Ok(
        BtcScriptBuilder::new()
            .push_slice(eth_address_and_nonce_hash.as_ref())
            .push_opcode(opcodes::all::OP_DROP)
            .push_opcode(opcodes::all::OP_DUP)
            .push_opcode(opcodes::all::OP_HASH160)
            .push_slice(
                &convert_btc_address_to_pub_key_hash_bytes(recipient)?[..]
            )
            .push_opcode(opcodes::all::OP_EQUALVERIFY)
            .push_opcode(opcodes::all::OP_CHECKSIG)
            .into_script()
    )
}

pub fn get_pbtc_deposit_address(cli_args: CliArgs) -> Result<String> {
    info!("✔ Creating pBTC deposit address...");
    State::init_from_cli_args(cli_args.clone())
        .and_then(get_btc_private_key_and_add_to_state)
        .and_then(get_eth_address_from_cli_args_and_put_in_state)
        .and_then(get_eth_address_and_nonce_hash_and_put_in_state)
        .and_then(|state|
            generate_pbtc_script_sig(
                &state.cli_args.arg_recipient,
                &state.get_btc_private_key()?.to_public_key_slice(),
                state.get_eth_address_and_nonce_hash()?
            ) 
        )
        .map(|script| 
            BtcAddress::p2sh(
                &script, 
                get_network_from_cli_arg(&cli_args.flag_network)
            ).to_string()
        )
}
