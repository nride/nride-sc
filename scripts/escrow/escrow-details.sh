#!/bin/sh

source ./scripts/util.sh

command="$CMD query wasm contract-state smart $ESCROW" 
command+=' {"details":{"id":"'"$1"'"}}'
command+=" --node $NODE"

echo $command

execute_tx_block "$command"
