#!/bin/sh

source ./scripts/util.sh

CODE=$1

command=($CMD tx wasm instantiate $CODE {}) 
command+=(--label "NRIDE ESCROW INIT")
command+=(--no-admin)
command+=(--from faucet)
command+=(--chain-id $CHAINID)
command+=(--gas-prices 0.1$FEETOKEN)
command+=(--gas auto)
command+=(--gas-adjustment 1.3)
command+=(--node $NODE)
command+=(-y)

execute_tx_block_2 "${command[@]}"