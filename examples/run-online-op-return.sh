#!/bin/bash
../target/release/bitcoff \
makeOnlineOpReturnTx \
mrASurhPLXdgoGQqCJgySF7QafooGvJL7Y \
4999 \
edB86cd455ef3ca43f0e227e00469C3bDFA40628 \
--network=Testnet \
--change=moBSQbHn7N9BC9pdtAMnA7GBiALzNMQJyE \
--fee=23 \
--outputPath="./make-online-op-return-output" \
--keyfile="./encrypted-btc-private-key.gpg" \
--logLevel="debug"
