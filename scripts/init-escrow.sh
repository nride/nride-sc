#!/bin/sh

set -eux

junod tx wasm instantiate 2 \
    '{}' \
    --amount 50000ujunox  \
    --label "NRIDE ESCROW INIT" \
    --no-admin \
    --from faucet \
    --chain-id testing \
    --gas-prices 0.1ujunox \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y 

# address: juno1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrq68ev2p