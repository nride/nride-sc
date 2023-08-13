#!/bin/sh

source ./scripts/util.sh

FROM=$1
ID=$2

json_msg='{"withdraw":{"id": "'"$ID"'"}}'

command=($CMD tx wasm execute $ESCROW)
command+=("$json_msg")
command+=(--from $FROM)
command+=(--fee-granter $($CMD keys show -a faucet))
command+=(--gas auto)
command+=(--gas-adjustment 1.3)
command+=(--gas-prices 0.1$FEETOKEN)
command+=(--chain-id $CHAINID)
command+=(--node $NODE)
command+=(-y)

execute_tx_block "${command[@]}"