#!/bin/sh

echo 'alice:'
$CMD q bank balances $($CMD keys show -a alice) --node $NODE
echo ''

echo 'bob:'
$CMD q bank balances $($CMD keys show -a bob) --node $NODE
echo ''

echo 'faucet:'
$CMD q bank balances $($CMD keys show -a faucet) --node $NODE
echo ''