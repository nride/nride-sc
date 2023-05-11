#!/bin/sh

set -eux

$CMD query wasm contract-state smart $ICS20 \
'{"channel":{"id":"'$1'"}}' \
--node $NODE