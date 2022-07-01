#!/bin/sh

CREATE_CONTENT='{'\
'"id": "first-escrow-id",'\
'"arbiter": "'"$(junod keys show -a tester)"'",'\
'"recipient": "'"$(junod keys show -a alice)"'",'\
'"title": "some_title",'\
'"cw20_whitelist": [],'\
'"description": "some_description"'\
'}'

ESCROW_EXECUTE='{ "create": '"$CREATE_CONTENT"'}'

MSG=$(echo "$ESCROW_EXECUTE" | base64)

SEND_CONTENT='{'\
'"contract": "juno1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrq68ev2p",'\
'"amount": "100",'\
'"msg": "'$MSG'"'\
'}'

TOKEN_EXECUTE='{"send": '"$SEND_CONTENT"'}'

echo $TOKEN_EXECUTE | jq 

junod tx wasm execute juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 \
"$TOKEN_EXECUTE" \
--from tester \
--gas 210000 \
--chain-id testing \
-b block \
-y