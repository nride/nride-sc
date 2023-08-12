#!/bin/sh

source ./scripts/util.sh

FROM=$1
NKN=$2
LOCATION=$3

json_msg='{"subscribe":{"nkn_addr":"'"$NKN"'","location":"'"$LOCATION"'"}}'

command=($CMD tx wasm execute $REGISTRY)
command+=("$json_msg")
command+=(--from $FROM)
command+=(--fee-granter $($CMD keys show -a faucet))
command+=(--gas auto)
command+=(--gas-adjustment 1.3)
command+=(--gas-prices 0.1$FEETOKEN)
command+=(--chain-id $CHAINID)
command+=(--node $NODE)
command+=(-y)

execute_tx_block_2 "${command[@]}"