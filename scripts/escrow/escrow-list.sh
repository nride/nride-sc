#!/bin/sh

source ./scripts/util.sh

command="$CMD query wasm contract-state smart $ESCROW"
command+=' {"list":{}}'
command+=" --node $NODE"

execute_tx_block "$command"
