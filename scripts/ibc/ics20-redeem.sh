#!/bin/sh

PORT="transfer"
FROM=$1
TO=$2
AMOUNT=$3
CHANNEL=$4

set -eux

osmosisd tx ibc-transfer transfer $PORT $CHANNEL $TO $AMOUNT$IBCDENOM \
--from $FROM \
--gas-prices 0.01uosmo \
--gas auto \
--gas-adjustment 1.3 \
--chain-id osmosis-1 \
--node $OSMONODE \
-b block \
-y \
--trace