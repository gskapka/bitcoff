# :fire: A BTC Transaction Maker

A simple Rust CLI for making BTC transactions given a __`UTXO`__ to spend.

&nbsp;

***

&nbsp;

### :point_right: Usage:

```

```

&nbsp;

***

&nbsp;

### :wrench: Build

__`❍ cargo build --release`__

&nbsp;

***

&nbsp;

### :radioactive: Critical Notes:

The tool relies on a GPG shell command in order to retrieve your BTC private key, and thus this only works wherever the __`gpg -d`__ command would.

It currently only supports the getting of a single UTXO for a transaction.

&nbsp;

***

&nbsp;

### :black_nib: Notes

The tool assumes you have a GPG encrypted BTC private key in __`WIF`__ format. You can pass in a path to the encrypted file when you run the CLI, else it'll default to __`./encrypted-btc-private-key`__ in the root of this repo.

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
 [ ] Make an offline version where use supplies UTXO deets!
 [ ] Make flag for traces since we might only want final output to stdout for programmatic usage.
 [ ] Value as a CLI arg, else we use the whole UTXO amount minus the fee.
 [ ] Fee as CLI arg.
 [ ] CLI arg for where to send the change to (default to address of private key used)
 [ ] Make 1st tx type an OP_RETURN w/ arbitrary data.
 [ ] Make endpoint configurable?
