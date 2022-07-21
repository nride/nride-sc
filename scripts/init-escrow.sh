#!/bin/sh

set -eux

$CMD tx wasm instantiate 2 \
    '{}' \
    --amount 50000$FEETOKEN  \
    --label "NRIDE ESCROW INIT" \
    --no-admin \
    --from faucet \
    --chain-id $CHAINID \
    --gas-prices 0.1$FEETOKEN \
    --gas auto \
    --gas-adjustment 1.3 \
    --node $NODE\
    -b block \
    -y 