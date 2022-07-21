#!/bin/sh

set -eux 

CREATE_CONTENT='{'\
'"id": "first-escrow-id",'\
'"arbiter": "'"$($CMD keys show -a faucet)"'",'\
'"recipient": "'"$($CMD keys show -a alice)"'",'\
'"title": "some_title",'\
'"cw20_whitelist": [],'\
'"description": "some_description"'\
'}'

ESCROW_EXECUTE='{ "create": '"$CREATE_CONTENT"'}'

MSG=$(echo "$ESCROW_EXECUTE" | base64)

# escrow contract
SEND_CONTENT='{'\
'"contract": "'$ESCROW'",'\
'"amount": "100",'\
'"msg": "'$MSG'"'\
'}'

TOKEN_EXECUTE='{"send": '"$SEND_CONTENT"'}'

echo $TOKEN_EXECUTE | jq 

# cw20 contract
$CMD tx wasm execute $NRIDE \
"$TOKEN_EXECUTE" \
--from faucet \
--gas 230000 \
--gas-prices 0.1$FEETOKEN \
--chain-id $CHAINID \
--node $NODE \
-b block \
-y