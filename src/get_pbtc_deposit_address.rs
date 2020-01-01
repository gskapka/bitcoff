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
        hash160,
        sha256d,
        sha256,
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

pub fn get_eth_address_from_cli_args_and_put_in_state(
    state: State
) -> Result<State> {
    convert_eth_address_to_bytes(&state.cli_args.arg_ethAddress[0])
        .and_then(|bytes| state.add_eth_address_bytes(bytes))
}


pub fn get_eth_address_and_nonce_hash(
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
        .and_then(|hash| {
            info!("✔ Eth address & nonce hash: {}", hex::encode(hash));
            state.add_eth_address_and_nonce_hash(hash)
        })
}

pub fn convert_btc_address_to_pub_key_hash_bytes(
    btc_address: &str
) -> Result<Bytes> {
    Ok(from_base58(btc_address)?[1..21].to_vec())
}

pub fn generate_pbtc_script_sig<'a>(
    recipient: &str, // FIXME: This is now unused!
    utxo_spender_pub_key_slice: &'a[u8],
    eth_address_and_nonce_hash: &sha256d::Hash,
) -> Result<BtcScript> {
    info!(
        "✔ Generating pBTC `script_sig` for recipient: {}",
        recipient,
    );
    debug!("Pub key: {}", hex::encode(utxo_spender_pub_key_slice));
    debug!("Eth address and nonce hash: {}", eth_address_and_nonce_hash);
    debug!("Eth address and nonce hash: {}", eth_address_and_nonce_hash);
    debug!(
        "Eth address and nonce hash bytes: {:?}", 
        eth_address_and_nonce_hash.as_ref()
    );
    debug!(
        "Test: {}", 
        hex::encode(&eth_address_and_nonce_hash[..]) // This is wrong endianess!
    );
    let script = BtcScriptBuilder::new()
        .push_slice(&eth_address_and_nonce_hash[..])
        .push_opcode(opcodes::all::OP_DROP)
        .push_slice(&utxo_spender_pub_key_slice)
        .push_opcode(opcodes::all::OP_CHECKSIG)
        .into_script();
    let script_serialized = btc_serialize(&script.clone());
    let script_serialized_2 = script.as_bytes().clone();
    let script_hash_160 = hash160::Hash::hash(&script_serialized);
    let script_2_hash_160 = hash160::Hash::hash(&script_serialized_2);
    let script_hash_256 = sha256d::Hash::hash(&script_serialized);
    let script_hash_both = hash160::Hash::hash(
        &sha256::Hash::hash(&script_serialized[2..])
    );
    let script_hash_both_double = hash160::Hash::hash(
        &sha256d::Hash::hash(&script_serialized[2..])
    );
    debug!("The script serialized: {}", hex::encode(script_serialized));
    debug!("The script serialized_2: {}", hex::encode(script_serialized_2));
    debug!("The script 160 hashed: {}", script_hash_160);
    debug!("The script 2 160 hashed: {}", script_2_hash_160);
    debug!("The script 256d hashed: {}", script_hash_256);
    debug!("The script 160(256) hashed: {}", script_hash_both);
    debug!("The script 160(256d) hashed: {}", script_hash_both_double);
    Ok(script)
}

// TODO We have this twice but slightly diff, in both pbtc things. FIXME
fn get_btc_script_and_put_in_state(state: State) -> Result<State> {
    info!("✔ Getting BTC redeem script and putting in state...");
    generate_pbtc_script_sig( // TODO Rename to "redeem script" or something? 
        &state.cli_args.arg_recipient,
        &state.get_btc_private_key()?.to_public_key_slice(),
        state.get_eth_address_and_nonce_hash()?
    ) 
        .and_then(|script| state.add_btc_script(script))
}

// NOTE: We should pass in the pub. key rather than deriving if from private!
// TODO: Add new CLI arg to solve above!
pub fn get_pbtc_deposit_address(cli_args: CliArgs) -> Result<String> {
    info!("✔ Creating pBTC deposit address...");
    State::init_from_cli_args(cli_args.clone())
        .and_then(get_btc_private_key_and_add_to_state)
        .and_then(get_eth_address_from_cli_args_and_put_in_state)
        .and_then(get_eth_address_and_nonce_hash_and_put_in_state)
        .and_then(get_btc_script_and_put_in_state)
        .and_then(|state| 
            Ok(
                BtcAddress::p2sh(
                    state.get_btc_script()?, 
                    get_network_from_cli_arg(&cli_args.flag_network)
                ).to_string()
            )
        )
}
