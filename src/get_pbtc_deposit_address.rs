use bitcoin::{
    hashes::{
        Hash,
        sha256d,
    },
    util::address::Address as BtcAddress,
    blockdata::{
        opcodes,
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
        Byte,
        Bytes,
        Result,
    },
    get_cli_args::{
        CliArgs,
        get_nonce_from_cli_arg,
        get_network_from_cli_arg,
    },
};

fn convert_eth_address_to_bytes(eth_address: &str) -> Result<Bytes> {
    Ok(hex::decode(&eth_address[..].replace("0x", ""))?)
}

pub fn get_eth_address_from_cli_args_and_put_in_state(state: State) -> Result<State> {
    convert_eth_address_to_bytes(&state.cli_args.arg_ethAddress[0])
        .and_then(|bytes| state.add_eth_address_bytes(bytes))
}


pub fn get_eth_address_and_nonce_hash(eth_address: &[Byte], nonce: &u64) -> Result<sha256d::Hash> {
    info!("✔ Getting ETH address & nonce hash from ETH address: 0x{} & nonce: {}", hex::encode(eth_address), nonce);
    let mut vec = vec![];
    vec.append(&mut eth_address.to_vec());
    vec.append(&mut nonce.to_le_bytes().to_vec());
    Ok(sha256d::Hash::hash(&vec))
}

fn get_eth_address_and_nonce_hash_and_put_in_state(state: State) -> Result<State> {
    get_eth_address_and_nonce_hash(state.get_eth_address_bytes()?, &get_nonce_from_cli_arg(&state.cli_args.flag_nonce)?)
        .and_then(|hash| {
            info!("✔ Eth address & nonce hash: {}", hex::encode(hash));
            state.add_eth_address_and_nonce_hash(hash)
        })
}

pub fn convert_btc_address_to_pub_key_hash_bytes(btc_address: &str) -> Result<Bytes> {
    Ok(from_base58(btc_address)?[1..21].to_vec())
}

pub fn generate_pbtc_script_sig<'a>(
    utxo_spender_pub_key_slice: &'a[u8],
    eth_address_and_nonce_hash: &sha256d::Hash,
) -> Result<BtcScript> {
    Ok(
        BtcScriptBuilder::new()
            .push_slice(&eth_address_and_nonce_hash[..])
            .push_opcode(opcodes::all::OP_DROP)
            .push_slice(&utxo_spender_pub_key_slice)
            .push_opcode(opcodes::all::OP_CHECKSIG)
            .into_script()
    )
}

// TODO We have this twice but slightly diff, in both pbtc things. FIXME
fn get_btc_script_and_put_in_state(state: State) -> Result<State> {
    info!("✔ Getting BTC redeem script and putting in state...");
    generate_pbtc_script_sig( // TODO Rename to "redeem script" or something?
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
            Ok(BtcAddress::p2sh(state.get_btc_script()?, get_network_from_cli_arg(&cli_args.flag_network)).to_string())
        )
}
