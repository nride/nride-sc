#!/bin/sh

$CMD query wasm contract-state smart $NRIDE \
'{"balance":{"address":"'"$1"'"}}' \
--node $NODE
