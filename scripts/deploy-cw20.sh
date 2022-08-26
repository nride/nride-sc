#!/bin/sh

$CMD tx wasm store cw-plus/contracts/cw20_base/artifactcw20_base.wasm  \
    --from faucet \
    --chain-id=$CHAINID \
    --gas-prices 0.1ujunox \
    --gas auto \
    --gas-adjustment 1.3 \
    --node $NODE\
    -b block \
    -y

## code id 1