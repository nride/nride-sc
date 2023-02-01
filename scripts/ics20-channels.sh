#!/bin/sh

set -eux

$CMD query wasm contract-state smart $ICS20 \
'{"list_channels":{}}' \
--node $NODE