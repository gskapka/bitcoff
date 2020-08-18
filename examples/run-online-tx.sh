#!/bin/bash
../target/release/btcoff \
makeOnlineTx \
3GvVZhsVswXR3vCkmQ4paXksa5eEo69axq \
5001 \
--network="Bitcoin" \
--change=19mLN6zzFsHQBmtBud4QtgJHtkNqcGEBjX \
--fee=216 \
--outputPath="./make-online-tx-output" \
--keyfile="my-mainnet-private-key-19mLN...EBjX.gpg" \
--logLevel="debug"
