#!/bin/sh

$CMD tx wasm store cw-plus/artifacts/cw20_base.wasm  \
    --from faucet \
    --chain-id=$CHAINID \
    --gas-prices 0.1ujunox \
    --gas auto \
    --gas-adjustment 1.3 \
    --node $NODE\
    -b block \
    -y

## code id 1