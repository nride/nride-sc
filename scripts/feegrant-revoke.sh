#//bin/sh

set -eux 

$CMD tx feegrant revoke $($CMD keys show -a faucet) $($CMD keys show -a $1) \
--from faucet \
--gas-prices 0.1$FEETOKEN \
--chain-id $CHAINID \
--node $NODE \
-b block \
-y \
