#!/bin/sh

source ./scripts/util.sh

PORT="transfer"
FROM=$1
TO=$2
AMOUNT=$3
CHANNEL=$4

command=(osmosisd tx ibc-transfer transfer)
command+=($PORT $CHANNEL $TO $AMOUNT$IBCDENOM)
command+=(--from $FROM)
command+=(--gas-prices 0.1uosmo)
command+=(--gas auto)
command+=(--gas-adjustment 1.3)
command+=(--chain-id osmosis-1)
command+=(--node $OSMONODE)
command+=(-y)

execute_tx_block_2 "${command[@]}"