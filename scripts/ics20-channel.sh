#!/bin/sh

set -eux

CHANNEL=$1

$CMD query wasm contract-state smart $ICS20 \
'{"channel":{"id":"'"$CHANNEL"'"}}' \
--node $NODE