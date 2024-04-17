#!/bin/sh

source ./scripts/util.sh

FROM=$1
ID=$2
USER_B=$3
LOCK=$4

CREATE_CONTENT='{'\
'"id": "'$ID'",'\
'"user_b": "'"$($CMD keys show -a $USER_B)"'",'\
'"lock": "'"$LOCK"'"'\
'}';

ESCROW_EXECUTE='{ "create": '"$CREATE_CONTENT"'}';

MSG=$(echo "$ESCROW_EXECUTE" | base64);

# escrow contract
SEND_CONTENT='{'\
'"contract": "'$ESCROW'",'\
'"amount": "100",'\
'"msg": "'$MSG'"'\
'}';

TOKEN_EXECUTE='{"send": '"$SEND_CONTENT"'}';

echo "$TOKEN_EXECUTE" 

# cw20 contract
command=($CMD tx wasm execute $NRIDE)
command+=("$TOKEN_EXECUTE")
command+=(--from $FROM)
command+=(--fee-granter $($CMD keys show -a faucet)) 
command+=(--gas-prices 0.1$FEETOKEN)
command+=(--gas auto)
command+=(--gas-adjustment 1.3)
command+=(--chain-id $CHAINID)
command+=(--node $NODE)
command+=(-y)

execute_tx_block "${command[@]}"