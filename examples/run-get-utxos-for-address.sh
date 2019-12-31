#!/bin/bash
../target/release/btc-tx-maker \
getUtxosForAddress \
2N88QH8W9iRLoW3sA4Ke1oFHwMshtMrUVos \
--network=Testnet \
--outputPath="./get-utxos-for-address-output" \
--logLevel="debug" 
