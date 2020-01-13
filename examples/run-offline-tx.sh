#!/bin/bash
../bin/btcoff \
makeOfflineTx \
mudzxCq9aCQ4Una9MmayvJVCF1Tj9fypiM \
666 \
mwqqpopcLxEFUCPGGPV7zpfm3xrSAjRcyq \
1337 \
2N88QH8W9iRLoW3sA4Ke1oFHwMshtMrUVos \
10000 \
--utxoFile="./get-utxos-output" \
--keyfile="./encrypted-btc-private-key.gpg" \
--network=Testnet \
--fee=23 \
--change=moBSQbHn7N9BC9pdtAMnA7GBiALzNMQJyE \
--outputPath="./make-offline-tx-output" \
--logLevel="debug"
