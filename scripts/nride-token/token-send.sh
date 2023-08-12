#!/bin/sh

source ./scripts/util.sh

FROM=$1
TO=$2
AMOUNT=$3

json_msg='{"transfer":{"amount":"'$AMOUNT'","recipient":"'"$($CMD keys show -a $TO)"'"}}' 

command=($CMD tx wasm execute $NRIDE)
command+=("$json_msg")
command+=(--from $FROM)
command+=(--gas-prices 0.1$FEETOKEN)
command+=(--gas auto)
command+=(--gas-adjustment 1.3)
command+=(--chain-id $CHAINID)
command+=(--node $NODE)
command+=(-y)

execute_tx_block_2 "${command[@]}"