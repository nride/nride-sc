#!/bin/sh

set -eux 

CONFIGFILE=$1
ACC=$2

osmosisd tx gamm create-pool \
--pool-file $CONFIGFILE \
--from $(osmosisd keys show -a $ACC) \
--gas-prices 0.1uosmo \
--gas auto \
--gas-adjustment 1.3 \
--chain-id osmosis-1 \
--node $OSMONODE \
-b block \
-y