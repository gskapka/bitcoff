pub static USAGE_INFO: &'static str = "
❍ BTC Transaction Maker ❍

    Copyright Greg Kapka 2019
    Questions: greg@kapka.co.uk

❍ Info ❍

A maker of BTC transactions!

❍ Usage ❍

Usage:  btc-tx-maker --help
        btc-tx-maker makeTx <btc_block_hash> <utxo_index>

Commands:

    makeTx              ❍ Do the thing!

    <btc_block_hash>    ❍ The hash of the block containing the UTXO to spend.

    <utxo_index>        ❍ The index of the UTXO in the block.

Options:

    --help               ❍ Show this message.

";
