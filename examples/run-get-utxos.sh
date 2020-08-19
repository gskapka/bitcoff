#!/bin/bash
../target/release/bitcoff \
getUtxos \
--network=Testnet \
--outputPath="./get-utxos-output" \
--keyfile="./encrypted-btc-private-key.gpg" \
--logLevel="debug" 
