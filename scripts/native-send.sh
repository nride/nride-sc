#!/bin/sh

$CMD tx bank send faucet $($CMD keys show -a $1) 100$FEETOKEN \
--gas-prices 0.1$FEETOKEN \
--chain-id $CHAINID \
--node $NODE \
-b block \
-y 
