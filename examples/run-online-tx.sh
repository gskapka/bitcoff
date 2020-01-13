#!/bin/bash
../bin/btcoff \
makeOnlineTx \
2NGCmjxtAYgYytghra759EywJ33pcqH9RZ1 \
6000 \
--network="testnet" \
--change=moBSQbHn7N9BC9pdtAMnA7GBiALzNMQJyE \
--fee=23 \
--outputPath="./make-online-tx-output" \
--keyfile="./encrypted-btc-private-key.gpg" \
--logLevel="debug"
