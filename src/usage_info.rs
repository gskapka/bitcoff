pub static USAGE_INFO: &'static str = "
❍ BTC Transaction Maker ❍

    Copyright Greg Kapka 2019
    Questions: greg@kapka.co.uk

❍ Info ❍

A maker of BTC transactions!

❍ Usage ❍

Usage:  btc-tx-maker --help
        btc-tx-maker makeTx <btc-block-hash> <utxo-index> [--keyfile=<path>] [--network=<string>]

Commands:

    makeTx              ❍ Do the thing!

    <btc-block-hash>    ❍ The hash of the block containing the UTXO to spend.

    <utxo-index>        ❍ The index of the UTXO in the block.

Options:

    --help               ❍ Show this message.

    --network=<string>   ❍ Btc network: Either `Bitcoin` or `Testnet`.
                           [default: Bitcoin]

    --keyfile=<path>     ❍ Path to GPG-encrypted BTC private key hex string.
                           [default: ./encrypted-btc-private-key.gpg]

";
