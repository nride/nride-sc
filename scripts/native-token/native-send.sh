#!/bin/sh

source ./scripts/util.sh

TO=$1
AMOUNT=$2

command=($CMD tx bank send)
command+=(faucet)
command+=("$($CMD keys show -a $TO)")
command+=($AMOUNT$FEETOKEN)
command+=(--gas-prices 0.1$FEETOKEN)
command+=(--chain-id $CHAINID)
command+=(--node $NODE) 
command+=(-y)

execute_tx_block "${command[@]}"