#!/bin/sh

junod tx wasm execute juno1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrq68ev2p \
'{"approve":{"id": "first-escrow-id"}}' \
--from faucet \
--chain-id testing \
-b block \
-y