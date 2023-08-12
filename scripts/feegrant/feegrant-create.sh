#//bin/sh

source ./scripts/util.sh

GRANTEE=$1

command=($CMD tx feegrant grant)
command+=($($CMD keys show -a faucet))
command+=($($CMD keys show -a $GRANTEE))
command+=(--from faucet)
command+=(--gas-prices 0.1$FEETOKEN)
command+=(--gas auto)
command+=(--gas-adjustment 1.3)
command+=(--chain-id $CHAINID)
command+=(--node $NODE)
command+=(-y)

execute_tx_block_2 "${command[@]}"