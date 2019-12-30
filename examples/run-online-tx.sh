#!/bin/bash
../target/release/btc-tx-maker \
makeOnlineTx \
mudzxCq9aCQ4Una9MmayvJVCF1Tj9fypiM \
1337 \
mwqqpopcLxEFUCPGGPV7zpfm3xrSAjRcyq \
666 \
2N88QH8W9iRLoW3sA4Ke1oFHwMshtMrUVos \
1 \
--network=Testnet \
--change=moBSQbHn7N9BC9pdtAMnA7GBiALzNMQJyE \
--fee=23 \
--outputPath="./make-online-tx-output" \
--keyfile="./encrypted-btc-private-key.gpg" \
--logLevel="debug"
