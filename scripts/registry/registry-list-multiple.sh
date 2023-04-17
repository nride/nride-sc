#!/bin/sh

set -eux

locs=$(echo '"'$1'"' | jq -c 'split(",")')

$CMD query wasm contract-state smart $REGISTRY \
'{"list_multiple":{"locations":'$locs'}}' \
--node $NODE
