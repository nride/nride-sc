#!/bin/sh

set -eux

FROM=$1

EXECUTE='{"allow":{"contract":"'$NRIDE'"}}'

$CMD tx wasm execute $ICS20 "$EXECUTE" \
--from $FROM \
--fee-account $($CMD keys show -a faucet) \
--gas-prices 0.1$FEETOKEN \
--gas auto \
--gas-adjustment 1.3 \
--chain-id $CHAINID \
--node $NODE \
-b block \
-y