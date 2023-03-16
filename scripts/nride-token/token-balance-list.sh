#!/bin/sh

echo 'faucet:'
$CMD query wasm contract-state smart $NRIDE \
'{"balance":{"address":"'"$($CMD keys show -a faucet)"'"}}' \
--node $NODE


echo ''

echo 'alice:'
$CMD query wasm contract-state smart $NRIDE \
'{"balance":{"address":"'"$($CMD keys show -a alice)"'"}}' \
--node $NODE

echo ''

echo 'bob:'
$CMD query wasm contract-state smart $NRIDE \
'{"balance":{"address":"'"$($CMD keys show -a bob)"'"}}' \
--node $NODE
