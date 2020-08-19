#!/bin/bash
../target/release/bitcoff \
makeOfflineOpReturnTx \
mudzxCq9aCQ4Una9MmayvJVCF1Tj9fypiM \
1 \
decaff \
--utxoFile="./get-utxos-output" \
--network=Testnet \
--change=moBSQbHn7N9BC9pdtAMnA7GBiALzNMQJyE \
--fee=23 \
--outputPath="./make-offline-op-return-output" \
--keyfile="./encrypted-btc-private-key.gpg" \
--logLevel="trace"
