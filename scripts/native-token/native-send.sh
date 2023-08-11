#!/bin/sh

source ./scripts/util.sh

TO=$1
AMOUNT=$2

command="$CMD tx bank send faucet $($CMD keys show -a $TO) $AMOUNT$FEETOKEN \
--gas-prices 0.1$FEETOKEN \
--chain-id $CHAINID \
--node $NODE \
-y"

execute_tx_block "$command"

