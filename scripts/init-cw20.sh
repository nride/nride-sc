#!/bin/sh

set -eux

CODE=$1

$CMD tx wasm instantiate $CODE \
    '{"name":"NRIDE Coin","symbol":"NRIDE","decimals":6,"initial_balances":[{"address":"'"$($CMD keys show -a faucet)"'","amount":"12345678900"}]}' \
    --label "NRIDE TOKEN INIT BALANCES" \
    --no-admin \
    --from faucet \
    --chain-id $CHAINID \
    --gas-prices 0.1$FEETOKEN \
    --gas auto \
    --gas-adjustment 1.3 \
    --node $NODE\
    -b block \
    -y 