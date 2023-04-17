#!/bin/sh

set -eux

CODE=$1

$CMD tx wasm instantiate $CODE \
    '{}' \
    --label "NRIDE REGISTRY INIT" \
    --admin $($CMD keys show -a faucet) \
    --from faucet \
    --chain-id $CHAINID \
    --gas-prices 0.1$FEETOKEN \
    --gas auto \
    --gas-adjustment 1.3 \
    --node $NODE\
    -b block \
    -y 