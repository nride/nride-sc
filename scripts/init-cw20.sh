#!/bin/sh

set -eux

junod tx wasm instantiate 1 \
    '{"name":"NRIDE Coin","symbol":"NRIDE","decimals":6,"initial_balances":[{"address":"'"$(junod keys show -a faucet)"'","amount":"12345678000"}]}' \
    --amount 50000ujunox  \
    --label "NRIDE TOKEN INIT BALANCES" \
    --no-admin \
    --from faucet \
    --chain-id testing \
    --gas-prices 0.1ujunox \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y 

# address: juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8