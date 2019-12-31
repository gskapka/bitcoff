pub static USAGE_INFO: &'static str = "
❍ BTC Transaction Maker ❍

    Copyright Greg Kapka 2019
    Questions: greg@kapka.co.uk

❍ Info ❍

A maker of BTC transactions!

❍ Usage ❍

Usage:  btcoff --help
        btcoff getPBTCDepositAddress <recipient> <ethAddress> [--nonce=<uint>] [--keyfile=<path>] [--network=<string>] [--outputPath=<path>] [--logLevel=<level>]
        btcoff getUtxos [--keyfile=<path>] [--network=<string>] [--outputPath=<path>] [--logLevel=<level>]
        btcoff getUtxosForAddress <btcAddress> [--network=<string>] [--outputPath=<path>] [--logLevel=<level>]
        btcoff makeOnlineTx (<to> <amount>)... [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--change=<string>] [--outputPath=<path>] [--logLevel=<level>]
        btcoff makeOfflineTx (<to> <amount>)... (--utxoFile=<path> | <utxos>) [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--change=<string>] [--outputPath=<path>] [--logLevel=<level>]
        btcoff makeOnlineOpReturnTx (<to> <amount>)... <data> [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--change=<string>] [--outputPath=<path>] [--logLevel=<level>]
        btcoff makeOfflineOpReturnTx (<to> <amount>)... <data> (--utxoFile=<path> | <utxos>) [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--change=<string>] [--outputPath=<path>] [--logLevel=<level>]

Commands:

    getPBTCDepositAddress ❍ Generate a BTC deposit address for a given ETH 
                            address for the Provable pBTC

    getUtxos              ❍ *Needs internet connection!* Makes API call to get 
                            all UTXOs associated with address derived from the
                            encrypted private key. UTXOs are presented in the
                            following JSON format:
                            [
                                {
                                    utxo_hex: <0x...>,
                                    utxo_value: <value-in-Satoshis>,
                                },...
                            ]

    getUtxosForAddress    ❍ *Needs internet connection!* Makes API call to get 
                            all UTXOs associated with supplied BTC address 
                            UTXOs are presented in the following JSON format:
                            [
                                {
                                    utxo_hex: <0x...>,
                                    utxo_value: <value-in-Satoshis>,
                                },...
                            ]

    makeOnlineTx          ❍ Create a simple BTC p2pkh transaction to one or more 
                            addresses. This online version will grab the UTXO 
                            set for the private key you provide via an API call.

    makeOfflineTx          ❍ Create a simple BTC transaction to one or more 
                            addresses. In this offline version, the UTXOs must 
                            be passed in via as either a JSON string, or from a 
                            file, both of which must use the JSON format:
                            [
                                {
                                    utxo_hex: <0x...>,
                                    utxo_value: <value-in-Satoshis>,
                                },...
                            ]

    makeOnlineOpReturnTx  ❍ Create an `OP_RETURN` transaction, pay the `to` 
                            address via a `p2pkh` transaction and where the 
                            `OP_RETURN` output contains the <data> supplied. In
                            this online version, available UTXOs for the address
                            of the private-key supplied are pulled from a block
                            explorer.

    makeOfflineOpReturnTx ❍ Create an `OP_RETURN` transaction, pay the `to` 
                            address via a `p2pkh` transaction and where the 
                            `OP_RETURN` output contains the <data> supplied. In
                            this offline version, the UTXOs required must be 
                            passed in via as either a JSON string, or from a 
                            file, both of which must use the same JSON format as 
                            the above `getUtxos` command returns:
                            [
                                {
                                    utxo_hex: <0x...>,
                                    utxo_value: <value-in-Satoshis>,
                                },...
                            ]

    <to>                  ❍ Address to send the transaction to.

    <amount>              ❍ Amount to send (in Satoshis).

    <data>                ❍ The hex data for the `OP_RETURN` output.

    <btcAddress>          ❍ A bitcoin address.

    <ethAddress>          ❍ An ethereum address, in hex format.

    <recipient>           ❍ The BTC recipient address.

    <utxos>               ❍ The UTXOs required for a BTC transaction, as a 
                            valid JSON string in the form:
                            [
                                {
                                    utxo_hex: <0x...>,
                                    utxo_value: <value-in-Satoshis>,
                                },...
                            ]

Options:

    --help               ❍ Show this message.

    --fee=<uint>         ❍ Fee to pay in Satoshis-per-byte.
                           [default: 23]

    --network=<string>   ❍ Btc network: Either `Bitcoin` or `Testnet`.
                           [default: Bitcoin]

    --outputPath=<path>  ❍ Save the tool's output to given path.

    --logLevel=<level>   ❍ Define the level of logging in the tool's output as
                           one of: `none`, `info`, `debug`, `trace` or `error`.
                           [default: info]

    --keyfile=<path>     ❍ Path to GPG-encrypted BTC private key in wallet 
                           import format (`WIF`).
                           [default: ./encrypted-btc-private-key.gpg]

    --nonce=<uint>       ❍ A nonce to be combined with the ETH address before
                           hashing. A nonce of '0' will use a unix timestamp 
                           instead. [default: 0]

    --change=<string>    ❍ Address to send any change to. Defaults to address 
                           of the private key used for the transaction.
                           [default: signer]

    --utxoFile=<path>    ❍ Path to a file containing a valid JSON array of BTC
                           UTXOs in the format:
                           [
                               {
                                   utxo_hex: <0x...>,
                                   utxo_value: <value-in-Satoshis>,
                               },...
                           ]
";
