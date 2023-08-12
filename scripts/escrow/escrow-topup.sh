#!/bin/sh

source ./scripts/util.sh

FROM=$1
ID=$2
LOCK=$3

TOPUP_CONTENT='{'\
'"id": "'$ID'",'\
'"account_a_lock": "'"$LOCK"'"'\
'}';

ESCROW_EXECUTE='{ "top_up": '"$TOPUP_CONTENT"'}';

MSG=$(echo "$ESCROW_EXECUTE" | base64);

# escrow contract
SEND_CONTENT='{'\
'"contract": "'$ESCROW'",'\
'"amount": "100",'\
'"msg": "'$MSG'"'\
'}';

TOKEN_EXECUTE='{"send": '"$SEND_CONTENT"'}';

echo $TOKEN_EXECUTE | jq

# cw20 contract
command="$CMD tx wasm execute $NRIDE \
"$TOKEN_EXECUTE" \
--from $FROM \
--fee-account $($CMD keys show -a faucet) \
--gas-prices 0.1$FEETOKEN \
--gas auto \
--gas-adjustment 1.3 \
--chain-id $CHAINID \
--node $NODE \
-b block \
-y"

execute_tx_block "$command"