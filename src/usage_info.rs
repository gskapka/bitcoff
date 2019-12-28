pub static USAGE_INFO: &'static str = "
❍ BTC Transaction Maker ❍

    Copyright Greg Kapka 2019
    Questions: greg@kapka.co.uk

❍ Info ❍

A maker of BTC transactions!

❍ Usage ❍

Usage:  btcoff --help
        btcoff getUtxos [--keyfile=<path>] [--network=<string>] [--outputPath=<path>]
        btcoff makeOnlineOpReturnTx (<to> <amount>)... <data> [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--change=<string>] [--outputPath=<path>]
        btcoff makeOfflineOpReturnTx (<to> <amount>)... <data> (--utxoFile=<path> | <utxos>) [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--change=<string>] [--outputPath=<path>]

Commands:

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

    makeOnlineOpReturnTx  ❍ Create an `OP_RETURN` transaction, pay the `to` 
                            address via a `p2pkh` transaction and where the 
                            `OP_RETURN` output contains the <data> supplied. In
                            this online version, available UTXOs for the address
                            of the private-key supplied are pulled from a block
                            explorer.

    <to>                  ❍ Address to send the transaction to.

    <amount>              ❍ Amount to send (in Satoshis).

    <data>                ❍ The hex data for the `OP_RETURN` output.

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

    --keyfile=<path>     ❍ Path to GPG-encrypted BTC private key in wallet 
                           import format (`WIF`).
                           [default: ./encrypted-btc-private-key.gpg]

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
