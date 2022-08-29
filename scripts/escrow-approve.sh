#!/bin/sh

$CMD tx wasm execute $ESCROW \
'{"approve":{"id": "first-escrow-id"}}' \
--from faucet \
--gas auto \
--gas-adjustment 1.3 \
--gas-prices 0.1$FEETOKEN \
--chain-id $CHAINID \
--node $NODE \
-b block \
-y