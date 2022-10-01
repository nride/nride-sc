#//bin/sh

set -eux 

GRANTEE=$1

$CMD tx feegrant grant $($CMD keys show -a faucet) $($CMD keys show -a $GRANTEE) \
--from faucet \
--gas-prices 0.1$FEETOKEN \
--gas auto \
--gas-adjustment 1.3 \
--chain-id $CHAINID \
--node $NODE \
-b block \
-y \
