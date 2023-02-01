#!/bin/sh

set -eux

FROM=$1
TO=$2
AMOUNT=$3
CHANNEL=$4


TRANSFER_CONTENT='{"channel":"'$CHANNEL'","remote_address":"'$TO'"}';

# TRANSFER_MSG='{"transferxx":'"$TRANSFER_CONTENT"'}';

# echo "$TRANSFER_MSG"

# MSG=$(echo "$TRANSFER_MSG" | base64);

MSG=$(echo "$TRANSFER_CONTENT" | base64);

SEND_CONTENT='{'\
'"contract":"'$ICS20'",'\
'"amount":"'$AMOUNT'",'\
'"msg":"'$MSG'"'\
'}';

TOKEN_EXECUTE='{"send":'"$SEND_CONTENT"'}';

echo $TOKEN_EXECUTE

$CMD tx wasm execute $NRIDE "$TOKEN_EXECUTE" \
--from $FROM \
--fee-account $($CMD keys show -a faucet) \
--gas-prices 0.1$FEETOKEN \
--gas auto \
--gas-adjustment 1.3 \
--chain-id $CHAINID \
--node $NODE \
-b block \
-y