#!/bin/sh

set -eux 

ID=$1
USER_B=$2
LOCK=$3

get_timestamp()
{
   echo $(date -v +$1M +%s)
}
    
T1=$(get_timestamp 2);
T2=$(get_timestamp 5);

CREATE_CONTENT='{'\
'"id": "'$ID'",'\
'"user_b": "'"$($CMD keys show -a $USER_B)"'",'\
'"t1_timeout": '$T1','\
'"t2_timeout": '$T2','\
'"account_b_lock": "'"$LOCK"'"'\
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

echo $TOKEN_EXECUTE | jq

# cw20 contract
$CMD tx wasm execute $NRIDE \
"$TOKEN_EXECUTE" \
--from faucet \
--gas-prices 0.1$FEETOKEN \
--gas auto \
--gas-adjustment 1.3 \
--chain-id $CHAINID \
--node $NODE \
-b block \
-y