#!/bin/bash
../target/release/btcoff \
getPBTCDepositAddress \
moBSQbHn7N9BC9pdtAMnA7GBiALzNMQJyE \
0xfEDFe2616EB3661CB8FEd2782F5F0cC91D59DCaC \
--nonce=1337 \
--network="testnet" \
--keyfile="./encrypted-btc-private-key.gpg" \
--logLevel="debug"
