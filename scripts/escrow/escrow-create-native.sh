#!/bin/sh

source ./scripts/util.sh

FROM=$1
ID=$2
USER_B=$3
AMOUNT=$4
DENOM=$5
LOCK=$6

CREATE_MSG='{'\
'"id": "'$ID'",'\
'"user_b": "'"$($CMD keys show -a $USER_B)"'",'\
'"lock": "'"$LOCK"'"'\
'}';

MSG='{"create": '"$CREATE_MSG"'}';

command=($CMD tx wasm execute $ESCROW)
command+=("$MSG")
command+=(--amount $AMOUNT$DENOM)
command+=(--from $FROM)
command+=(--fee-granter $($CMD keys show -a faucet)) 
command+=(--gas-prices 0.1$FEETOKEN)
command+=(--gas auto)
command+=(--gas-adjustment 1.3)
command+=(--chain-id $CHAINID)
command+=(--node $NODE)
command+=(-y)

execute_tx_block "${command[@]}"