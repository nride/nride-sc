#!/bin/sh

$CMD query wasm contract-state smart $ESCROW \
'{"list":{}}' \
--node $NODE
