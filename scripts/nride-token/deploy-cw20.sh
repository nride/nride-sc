#!/bin/sh

$CMD tx wasm store cw-plus/artifacts/cw20_base.wasm  \
    --from faucet \
    --chain-id=$CHAINID \
    --gas-prices 1ujunox \
    --gas auto \
    --gas-adjustment 10 \
    --node $NODE \
    -y