#!/bin/sh

source ./scripts/util.sh

FROM=$1
ID=$2

command="$CMD tx wasm execute $ESCROW \
'{"withdraw":{"id": "'"$ID"'"}}' \
--from $FROM \
--fee-account $($CMD keys show -a faucet) \
--gas auto \
--gas-adjustment 1.3 \
--gas-prices 0.1$FEETOKEN \
--chain-id $CHAINID \
--node $NODE \
-b block \
-y"

execute_tx_block "$command"