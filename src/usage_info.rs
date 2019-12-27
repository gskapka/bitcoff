pub static USAGE_INFO: &'static str = "
❍ BTC Transaction Maker ❍

    Copyright Greg Kapka 2019
    Questions: greg@kapka.co.uk

❍ Info ❍

A maker of BTC transactions!

❍ Usage ❍

Usage:  btcoff --help
        btcoff getUtxos [--keyfile=<path>] [--network=<string>]
        btcoff makeOnlineOpReturnTx (<to> <amount>)... <data> [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--change=<string>]

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

    makeOnlineOpReturnTx  ❍ Create an `OP_RETURN` transaction, pay the `to` 
                            address via a `p2pkh` transaction and where the 
                            `OP_RETURN` output contains the <data> supplied. In
                            this online version, available UTXOs for the address
                            of the private-key supplied are pulled from a block
                            explorer.

    <to>                  ❍ Address to send the transaction to.

    <amount>              ❍ Amount to send (in Satoshis).

    <data>                ❍ The hex data for the `OP_RETURN` output.

Options:

    --help               ❍ Show this message.

    --fee=<uint>         ❍ Fee to pay in Satoshis-per-byte.
                           [default: 23]

    --network=<string>   ❍ Btc network: Either `Bitcoin` or `Testnet`.
                           [default: Bitcoin]

    --keyfile=<path>     ❍ Path to GPG-encrypted BTC private key in wallet 
                           import format (`WIF`).
                           [default: ./encrypted-btc-private-key.gpg]

    --change=<string>    ❍ Address to send any change to. Defaults to address 
                           of the private key used for the transaction.
                           [default: signer]
";
