#!/bin/sh

source ./scripts/util.sh

FROM=$1
TO=$2

json_msg='{"transfer":{"amount":"10","recipient":"'"$($CMD keys show -a $TO)"'"}}'

command=($CMD tx wasm execute)
command+=($NRIDE)
command+=("$json_msg")
command+=(--from $FROM)
command+=(--gas-prices 0.1$FEETOKEN)
command+=(--fee-granter $($CMD keys show -a faucet))
command+=(--chain-id $CHAINID)
command+=(--node $NODE)
command+=(-y)

execute_tx_block "${command[@]}"