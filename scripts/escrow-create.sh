#!/bin/sh

set -eux 

FROM=$1
ID=$2
USER_B=$3
LOCK=$4

# return a unix timestamp for NOW = x minutes (where x is the argument provided)
get_timestamp()
{
   echo $(date -v +$1M +%s)
}
    
T1=$(get_timestamp 2); # NOW + 2 minutes
T2=$(get_timestamp 5); # NOW + 5 minutes

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
--from $FROM \
--fee-account $($CMD keys show -a faucet) \
--gas-prices 0.1$FEETOKEN \
--gas auto \
--gas-adjustment 1.3 \
--chain-id $CHAINID \
--node $NODE \
-b block \
-y