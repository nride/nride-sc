#!/bin/sh

junod tx wasm store contracts/cw20_escrow.wasm  \
    --from faucet \
    --chain-id=testing \
    --gas-prices 0.1ujunox \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y