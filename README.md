# :fire: An Offline (|| Online!) BTC Transaction Maker

A simple Rust CLI for creating on- & offline BTC transactions given a (or some!) __`UTXO`__ to spend. Offline mode(s) makes zero API calls and thus can happily be run on an air-gapped machine.

&nbsp;

***

&nbsp;

### :point_right: Usage:

The __`./examples`__ directory has - unsurprisingly - some examples in it that you can examine to see how to run the tool. Before running the examples, you'll need to build the tool. You'll also need a gpg-encrypted private key. See the relevant sections of this __`README`__ for instructions.

Otherwise, the usage-notes are as follows:

```
❍ Bitcoff ❍

    Copyright Greg Kapka 2019
    Questions: greg@kapka.co.uk

❍ Info ❍

An on or offline BTC transaction signer!

❍ Usage ❍

Usage:  bitcoff --help
        bitcoff version
        bitcoff getUtxos [--keyfile=<path>] [--network=<string>] [--outputPath=<path>] [--logLevel=<level>]
        bitcoff getUtxosForAddress <btcAddress> [--network=<string>] [--outputPath=<path>] [--logLevel=<level>]
        bitcoff makeOnlineTx (<to> <amount>)... [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--change=<string>] [--outputPath=<path>] [--logLevel=<level>]
        bitcoff makeOfflineTx (<to> <amount>)... (--utxoFile=<path> | <utxos>) [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--change=<string>] [--outputPath=<path>] [--logLevel=<level>]
        bitcoff makeOnlineOpReturnTx (<to> <amount>)... <data> [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--change=<string>] [--outputPath=<path>] [--logLevel=<level>]
        bitcoff makeOfflineOpReturnTx (<to> <amount>)... <data> (--utxoFile=<path> | <utxos>) [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--change=<string>] [--outputPath=<path>] [--logLevel=<level>]

Commands:

    version               ❍ Show version info.
    getUtxos              ❍ Makes API call to get all UTXOs associated with address derived from the encrypted private
                            key. UTXOs are presented in the following JSON format:
                            [{ serialized_utxo: <0x...>, value: <value-in-Satoshis> },...]
    getUtxosForAddress    ❍ Makes API call to get all UTXOs associated with supplied BTC address UTXOs are presented in
                            the following JSON format:
                            [{ serialized_utxo: <0x...>, value: <value-in-Satoshis> },...]
    makeOnlineTx          ❍ Create a simple BTC p2pkh transaction to one or more addresses. This online version will
                            grab the UTXO set for the private key you provide via an API call.
    makeOfflineTx         ❍ Create a simple BTC transaction to one or more addresses. In this offline version, the
                            UTXOs must be passed in via as either a JSON string, or from a file, both of which must use
                            the JSON format:
                            [{ serialized_utxo: <0x...>, value: <value-in-Satoshis> },...]
    makeOnlineOpReturnTx  ❍ Create an `OP_RETURN` transaction, pay the `to` address via a `p2pkh` transaction and where
                            the `OP_RETURN` output contains the <data> supplied. In this online version, available UTXOs
                            for the address of the private-key supplied are pulled from a block explorer.
    makeOfflineOpReturnTx ❍ Create an `OP_RETURN` transaction, pay the `to` address via a `p2pkh` transaction and where
                            the `OP_RETURN` output contains the <data> supplied. In this offline version, the UTXOs
                            required must be passed in via as either a JSON string, or from a file, both of which must
                            use the same JSON format as the above `getUtxos` command returns:
                            [{ serialized_utxo: <0x...>, value: <value-in-Satoshis> },...]
    <to>                  ❍ Address to send the transaction to.
    <amount>              ❍ Amount to send (in Satoshis).
    <data>                ❍ The hex data for the `OP_RETURN` output.
    <btcAddress>          ❍ A bitcoin address.
    <utxos>               ❍ The UTXOs required for a BTC transaction, as a
                            valid JSON string in the form:
                            [{ serialized_utxo: <0x...>, value: <value-in-Satoshis> },...]

Options:

    --help                ❍ Show this message.
    --outputPath=<path>   ❍ Save the tool's output to given path.
    --fee=<uint>          ❍ Fee to pay in Satoshis-per-byte. [default: 23]
    --network=<string>    ❍ Btc network: Either `Bitcoin` or `Testnet`. [default: Bitcoin]
    --logLevel=<level>    ❍ Define the level of logging in the tool's output as one of: `none`, `info`, `debug`, `trace`
                            or `error` [default: none]
    --keyfile=<path>      ❍ Path to GPG-encrypted BTC private key in wallet import format (`WIF`).
                            [default: ./encrypted-btc-private-key.gpg]
    --nonce=<uint>        ❍ A nonce to be combined with the ETH address before hashing. A nonce of '0' will use a unix
                            timestamp instead. [default: 0]
    --change=<string>     ❍ Address to send any change to. Defaults to address of the private key used for the
                            transaction. [default: signer]
    --utxoFile=<path>     ❍ Path to a file containing a valid JSON array of BTC UTXOs in the format:
                            [{ serialized_utxo: <0x...>, value: <value-in-Satoshis> },...]
```

&nbsp;

***

&nbsp;

### :wrench: Build It

__`❍ cargo +nightly build --release`__

Then you'll find the binary @ __`./target/release/bitcoff`__ to do with as you please.

&nbsp;

***

&nbsp;

### :radioactive: Critical Notes:

The tool relies on a GPG shell command in order to retrieve your BTC private key, and thus this only works wherever the __`gpg -d`__ command would.

The tool currently gathers ALL of an address' UTXOs for the transaction, regardless of value. Some more finessing on this point will/might be available soon :P

&nbsp;

***

&nbsp;

### :black_nib: Notes

The tool assumes you have a GPG encrypted BTC private key in __`WIF`__ format. You can pass in a custom path to your encrypted keyfile when you run the CLI, else it'll default to __`./encrypted-btc-private-key`__ in the root of this repo.

&nbsp;

***

&nbsp;

### :guardsman: Tests

To run the tests simply run:

__`❍ cargo +nightly test`__

&nbsp;

***

&nbsp;

### :black_nib: To Do:
 [x] Make an offline version where use supplies UTXO details in hex format!
 [x] Make flag for traces since we might only want final output to stdout for programmatic usage.
 [x] Value as a CLI arg, else we use the whole UTXO amount minus the fee.
 [x] Fee as CLI arg.
 [x] CLI arg for where to send the change to (default to address of private key used)
 [x] Make 1st tx type an OP_RETURN w/ arbitrary data.
 [x] Have online version pull UTXO list from API, then pull each tx and get the utxos that way.
 [x] When implementing the above, option to use ALL utxos maybe to sweep them up into a single one for future use?
 [ ] Make endpoint configurable?
 [ ] Have a flag which when used will use only the first address supplied and will ignore the amount and instead sweep the full balance.
