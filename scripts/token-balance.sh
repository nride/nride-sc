#!/bin/sh

echo 'tester:'
junod query wasm contract-state smart juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 \
'{"balance":{"address":"'"$(junod keys show -a tester)"'"}}'

echo ''

echo 'alice:'
junod query wasm contract-state smart juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 \
'{"balance":{"address":"'"$(junod keys show -a alice)"'"}}'