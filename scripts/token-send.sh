#!/bin/sh

junod tx wasm execute juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 \
'{"transfer":{"amount":"200","recipient":"'"$(junod keys show -a alice)"'"}}' \
--from faucet \
--chain-id testing \
-b block \
-y