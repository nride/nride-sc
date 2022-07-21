#!/bin/sh

set -eux

FROM=$1
TO=$2

$CMD tx wasm execute $NRIDE \
'{"transfer":{"amount":"100","recipient":"'"$($CMD keys show -a $TO)"'"}}' \
--from $FROM \
--gas-prices 0.1$FEETOKEN \
--chain-id $CHAINID \
--node $NODE \
-b block \
-y