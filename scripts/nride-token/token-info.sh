#!/bin/sh

set -eux

$CMD query wasm contract-state smart $NRIDE \
'{"token_info":{}}' \
--node $NODE
