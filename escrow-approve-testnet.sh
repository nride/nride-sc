#!/bin/sh

junod tx wasm execute juno1eds9t7rpfsfeyu35nevyc8tglumvejjg6p0yegkrv4wjlf0lghtqwy75uv \
'{"approve":{"id": "first-escrow-id"}}' \
--from faucet \
--chain-id uni-3 \
--gas 210000 \
--gas-prices 0.1ujunox \
-b block \
-y \
--node https://rpc.uni.juno.deuslabs.fi:443
