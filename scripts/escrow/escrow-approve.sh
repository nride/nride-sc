#!/bin/sh

source ./scripts/util.sh

FROM=$1
ID=$2
SECRET=$3

command="$CMD tx wasm execute $ESCROW \
'{"approve":{"id": "'"$ID"'", "secret": "'"$SECRET"'"}}' \
--from $FROM \
--fee-account $($CMD keys show -a faucet) \
--gas auto \
--gas-adjustment 1.3 \
--gas-prices 0.1$FEETOKEN \
--chain-id $CHAINID \
--node $NODE \
-y"

execute_tx_block "$command"