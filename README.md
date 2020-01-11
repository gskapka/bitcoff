# :fire: An Offline (|| Online!) BTC Transaction Maker

A simple Rust CLI for creating on- & offline BTC transactions given a __`UTXO`__ to spend. Offline mode(s) makes zero API calls and thus can happily be run on an air-gapped machine.

&nbsp;

***

&nbsp;

### :point_right: Usage:

The __`./examples`__ directory has - unsurprisingly - some examples in it that you can examine to see how to run the tool. Otherwise, the usage-notes are as follows:

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

The tool currently gathers ALL of a key's UTXOs for the transaction, regardless of value. Some more finessing on this point will be available soon :P

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
 [ ] Make an offline version where use supplies UTXO details in hex format!
 [ ] Make flag for traces since we might only want final output to stdout for programmatic usage.
 [ ] Value as a CLI arg, else we use the whole UTXO amount minus the fee.
 [ ] Fee as CLI arg.
 [ ] CLI arg for where to send the change to (default to address of private key used)
 [ ] Make 1st tx type an OP_RETURN w/ arbitrary data.
 [ ] Make endpoint configurable?
 [ ] Have online version pull UTXO list from API, then pull each tx and get the utxos that way.
 [ ] When implementing the above, option to use ALL utxos maybe to sweep them up into a single one for future use?
 [ ] Have a flag which when used will use only the first address supplied and will ignore the amount and instead sweep the full balance.
