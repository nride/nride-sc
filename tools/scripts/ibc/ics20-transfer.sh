#!/bin/sh

source ./scripts/util.sh

FROM=$1
TO=$2
AMOUNT=$3
CHANNEL=$4

TRANSFER_CONTENT='{"channel":"'$CHANNEL'","remote_address":"'$TO'"}';

MSG=$(echo "$TRANSFER_CONTENT" | base64);

SEND_CONTENT='{'\
'"contract":"'$ICS20'",'\
'"amount":"'$AMOUNT'",'\
'"msg":"'$MSG'"'\
'}';

TOKEN_EXECUTE='{"send":'"$SEND_CONTENT"'}';

echo $TOKEN_EXECUTE

command=($CMD tx wasm execute $NRIDE)
command+=("$TOKEN_EXECUTE")
command+=(--from $FROM)
command+=(--gas-prices 0.1$FEETOKEN)
command+=(--gas auto)
command+=(--gas-adjustment 1.3)
command+=(--chain-id $CHAINID)
command+=(--node $NODE)
command+=(-y)

execute_tx_block_2 "${command[@]}"