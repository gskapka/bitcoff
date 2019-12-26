pub static USAGE_INFO: &'static str = "
❍ BTC Transaction Maker ❍

    Copyright Greg Kapka 2019
    Questions: greg@kapka.co.uk

❍ Info ❍

A maker of BTC transactions!

❍ Usage ❍

Usage:  btc-tx-maker --help
        btc-tx-maker makeOpReturnTx <to> <data> (<tx-id> <utxo-indices>)... [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--change=<string>] [--value=<uint>] 

Commands:

    makeOpReturnTx        ❍ Create an `OP_RETURN` transaction, pay the `to` 
                            address via a `p2pkh` transactin and where the 
                            `OP_RETURN` output contains the <data> supplied.

    <to>                  ❍ Address to send the transaction to.

    <data>                ❍ The hex data for the `OP_RETURN` output.

    <tx-id>               ❍ The transaction hash(es) of the transaction(s) that
                            contain the UTXO output(s) you wish to spend.

    <utxo-indices>        ❍ The index/indices of the UTXO(s) in the above
                            transactions.

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

    --value=<string>     ❍ Amount of BTC to send in Satoshis. Must be < total 
                           UTXO values plus fee. If omitted, the tool defaults 
                           to: `total UTXO value - fee`
                           [default: max]

";
