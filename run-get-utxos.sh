#!/bin/bash
./target/release/btc-tx-maker getUtxos \
--network=Testnet \
--outputPath="./get-utxos-output" \
--logLevel="debug"
