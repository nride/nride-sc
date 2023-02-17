#!/bin/sh

$CMD query wasm contract-state smart $REGISTRY \
'{"list":{"location":"'"$1"'"}}' \
--node $NODE
