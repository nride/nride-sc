#!/bin/sh

set -eux

FROM=$1
TO=$2

$CMD tx wasm execute $NRIDE \
'{"transfer":{"amount":"10","recipient":"'"$($CMD keys show -a $TO)"'"}}' \
--from $FROM \
--gas-prices 1$FEETOKEN \
--fee-account $($CMD keys show -a faucet) \
--chain-id $CHAINID \
--node $NODE \
-b block \
-y