#!/bin/sh

$CMD tx wasm store contracts/cw20_escrow.wasm  \
    --from faucet \
    --chain-id=$CHAINID \
    --gas-prices 0.1uconst \
    --gas auto \
    --gas-adjustment 1.3 \
    --node $NODE\
    -b block \
    -y

    ## code id 2