#!/bin/sh

echo 'faucet:'
junod query wasm contract-state smart juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 \
'{"balance":{"address":"'"$(junod keys show -a faucet)"'"}}'

echo ''

echo 'alice:'
junod query wasm contract-state smart juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 \
'{"balance":{"address":"'"$(junod keys show -a alice)"'"}}'

echo 'bob:'
junod query wasm contract-state smart juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 \
'{"balance":{"address":"'"$(junod keys show -a bob)"'"}}'