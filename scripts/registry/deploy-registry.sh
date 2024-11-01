#!/bin/sh

source ./scripts/util.sh

command=($CMD tx wasm store)
command+=(cw-nride-registry/artifacts/cw_nride_registry.wasm)
command+=(--from faucet)
command+=(--chain-id=$CHAINID)
command+=(--gas-prices 0.1$FEETOKEN)
command+=(--gas auto)
command+=(--gas-adjustment 1.3)
command+=(--node $NODE)
command+=(-y)

execute_tx_block "${command[@]}"