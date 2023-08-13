#!/bin/sh

source ./scripts/util.sh

CODE=$1

json_msg='{"name":"NRIDE Coin","symbol":"NRIDE","decimals":6,"initial_balances":[{"address":"'"$($CMD keys show -a faucet)"'","amount":"12345678900"}]}'

command=($CMD tx wasm instantiate)
command+=($CODE)
command+=("$json_msg")
command+=(--label "NRIDE TOKEN INIT BALANCES")
command+=(--no-admin)
command+=(--from faucet) 
command+=(--chain-id $CHAINID)
command+=(--gas-prices 0.1$FEETOKEN)
command+=(--gas auto)
command+=(--gas-adjustment 1.3)
command+=(--node $NODE)
command+=(-y)

execute_tx_block "${command[@]}"