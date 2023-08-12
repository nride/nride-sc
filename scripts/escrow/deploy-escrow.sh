#!/bin/sh

source ./scripts/util.sh

command="$CMD tx wasm store cw-nride-escrow/artifacts/cw_nride_escrow.wasm  \
    --from faucet \
    --chain-id=$CHAINID \
    --gas-prices 0.1$FEETOKEN \
    --gas auto \
    --gas-adjustment 1.3 \
    --node $NODE \
    -y"

execute_tx_block "$command"