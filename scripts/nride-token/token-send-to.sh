#!/bin/sh

set -eux

FROM=$1
TO=$2
AMOUNT=$3

$CMD tx wasm execute $NRIDE \
'{"transfer":{"amount":"'$AMOUNT'","recipient":"'"$TO"'"}}' \
--from $FROM \
--gas-prices 0.1$FEETOKEN \
--gas auto \
--gas-adjustment 1.3 \
--chain-id $CHAINID \
--node $NODE \
-b block \
-y