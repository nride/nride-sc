#!/bin/sh

junod tx wasm instantiate 1 \
    '{"name":"NRIDE Coin","symbol":"NRIDE","decimals":6,"initial_balances":[{"address":"juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y","amount":"12345678000"}]}' \
    --amount 50000ujunox  \
    --label "NRIDE TOKEN INIT BALANCES" \
    --no-admin \
    --from tester \
    --chain-id testing \
    --gas-prices 0.1ujunox \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y 