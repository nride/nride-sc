#!/bin/sh

$CMD query wasm contract-state smart $REGISTRY \
'{"details":{"address":"'"$1"'"}}' \
--node $NODE
