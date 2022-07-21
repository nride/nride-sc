#!/bin/sh

$CMD tx wasm execute $ESCROW \
'{"approve":{"id": "first-escrow-id"}}' \
--from faucet \
--gas 230000 \
--gas-prices 0.1$FEETOKEN \
--chain-id $CHAINID \
--node $NODE \
-b block \
-y