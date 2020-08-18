#!/bin/bash
../target/release/btcoff \
makePBTCUtxoTx \
0xfEDFe2616EB3661CB8FEd2782F5F0cC91D59DCaC \
1337 \
mudzxCq9aCQ4Una9MmayvJVCF1Tj9fypiM \
1337 \
--utxoFile="./get-utxos-for-address-output" \
--keyfile="./encrypted-btc-private-key.gpg" \
--network="testnet" \
--fee=23 \
--change=moBSQbHn7N9BC9pdtAMnA7GBiALzNMQJyE \
--outputPath="./make-pbtc-utxo-tx-output" \
--logLevel="trace"
