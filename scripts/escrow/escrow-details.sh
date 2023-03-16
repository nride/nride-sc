#!/bin/sh

$CMD query wasm contract-state smart $ESCROW \
'{"details":{"id":"'"$1"'"}}' \
--node $NODE
