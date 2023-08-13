#!/bin/sh

set -eux 

ACC=$1
AMOUNT=$2
GAUGE_ID=$3

osmosisd tx incentives add-to-gauge \
$GAUGE_ID \
$AMOUNT$IBCDENOM \
--from $(osmosisd keys show -a $ACC) \
--gas-prices 0.1uosmo \
--gas auto \
--gas-adjustment 1.3 \
--chain-id osmosis-1 \
--node $OSMONODE \
-b block \
-y