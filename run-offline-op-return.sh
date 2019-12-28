#!/bin/bash
./target/release/btc-tx-maker \
makeOfflineOpReturnTx \
mudzxCq9aCQ4Una9MmayvJVCF1Tj9fypiM \
666 \
decaff \
--utxoFile="./get-utxos-output" \
--network=Testnet \
--change=moBSQbHn7N9BC9pdtAMnA7GBiALzNMQJyE \
--fee=23 \
--outputPath="./make-offline-op-return-output" \
--logLevel="trace"
