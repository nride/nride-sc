#!/bin/sh

set -eux

ADDR=$1
CODE=$2

$CMD tx wasm migrate $ADDR $CODE '{}' \
    --from faucet \
    --chain-id $CHAINID \
    --gas-prices 0.1$FEETOKEN \
    --gas auto \
    --gas-adjustment 1.3 \
    --node $NODE\
    -b block \
    -y 