#!/bin/bash
../target/release/btc-tx-maker \
getUtxosForAddress \
2N2LHYbt8K1KDBogd6XUG9VBv5YM6xefdM2 \
--network="testnet" \
--outputPath="./get-utxos-for-address-output" \
--logLevel="debug" 
