#!/bin/sh

FROM=$1
NKN=$2
LOCATION=$3

$CMD tx wasm execute $REGISTRY \
'{"subscribe":{"nkn_addr": "'"$NKN"'", "location": "'"$LOCATION"'"}}' \
--from $FROM \
--fee-account $($CMD keys show -a faucet) \
--gas auto \
--gas-adjustment 1.3 \
--gas-prices 0.1$FEETOKEN \
--chain-id $CHAINID \
--node $NODE \
-b block \
-y