#!/bin/bash
../target/release/btc-tx-maker \
getUtxos \
--network=Testnet \
--outputPath="./get-utxos-output" \
--keyfile="./encrypted-btc-private-key.gpg" \
--logLevel="debug" 
