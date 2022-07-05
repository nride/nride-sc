#!/bin/sh

CREATE_CONTENT='{'\
'"id": "first-escrow-id",'\
'"arbiter": "'"$(junod keys show -a faucet)"'",'\
'"recipient": "'"$(junod keys show -a alice)"'",'\
'"title": "some_title",'\
'"cw20_whitelist": [],'\
'"description": "some_description"'\
'}'

ESCROW_EXECUTE='{ "create": '"$CREATE_CONTENT"'}'

MSG=$(echo "$ESCROW_EXECUTE" | base64)

SEND_CONTENT='{'\
'"contract": "juno1eds9t7rpfsfeyu35nevyc8tglumvejjg6p0yegkrv4wjlf0lghtqwy75uv",'\
'"amount": "0ujunox",'\
'"msg": "'$MSG'"'\
'}'


TOKEN_EXECUTE='{"send": '"$SEND_CONTENT"'}'

echo $TOKEN_EXECUTE | jq 

junod tx wasm execute juno1caapzpyuhddkzps9nwatyknlvmm2av6whkk7aqse4umzmp0gpm5se7nzg7 \
"$TOKEN_EXECUTE" \
--from faucet \
--chain-id=uni-3 \
--gas 223000 \
--gas-prices 1ujunox \
-b block \
-y \
--node https://rpc.uni.juno.deuslabs.fi:443