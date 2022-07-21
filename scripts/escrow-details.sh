#!/bin/sh

$CMD query wasm contract-state smart $ESCROW \
'{"details":{"id":"first-escrow-id"}}' \
--node $NODE
