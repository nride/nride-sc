#!/bin/sh

TO=$1
AMOUNT=$2

$CMD tx bank send faucet $($CMD keys show -a $TO) $AMOUNT$FEETOKEN \
--gas-prices 0.1$FEETOKEN \
--chain-id $CHAINID \
--node $NODE \
-b block \
-y 
