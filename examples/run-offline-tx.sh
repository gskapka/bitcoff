#!/bin/bash
../target/release/btc-tx-maker \
makeOfflineTx \
mudzxCq9aCQ4Una9MmayvJVCF1Tj9fypiM \
666 \
mwqqpopcLxEFUCPGGPV7zpfm3xrSAjRcyq \
1337 \
--utxoFile="./get-utxos-output" \
--network=Testnet \
--change=moBSQbHn7N9BC9pdtAMnA7GBiALzNMQJyE \
--fee=23 \
--outputPath="./make-offline-tx-output" \
--keyfile="./encrypted-btc-private-key.gpg" \
--logLevel="debug"
