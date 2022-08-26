#!/bin/sh

$CMD tx wasm store cw-i4i/artifacts/cw20_i4i.wasm  \
    --from faucet \
    --chain-id=$CHAINID \
    --gas-prices 0.1$FEETOKEN \
    --gas auto \
    --gas-adjustment 1.3 \
    --node $NODE\
    -b block \
    -y