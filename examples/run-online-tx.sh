#!/bin/bash
../bin/btcoff \
makeOnlineTx \
2Mygt1EwSJHbX8L4qLKzfTc4ZnykDzxFyAa \
5001 \
--network="Testnet" \
--change=moBSQbHn7N9BC9pdtAMnA7GBiALzNMQJyE \
--fee=20 \
--outputPath="./make-online-tx-output" \
--keyfile="encrypted-btc-private-key.gpg" \
--logLevel="debug"
