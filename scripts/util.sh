#!/bin/sh

# broadcast a tx and wait for it to be committed
function execute_tx_block() {
    a=("$@")
    ((last_idx=${#a[@]} - 1))
    cmd="${a[0]}"
    args=()
    for i in `seq 1 $last_idx` ; do
        args+=("${a[$i]}")
    done
    
    if txres=$($cmd "${args[@]}" 2>&1); then
        txcode=$(echo "$txres" | awk '/code:/ {print $2}')
        if [ "$txcode" -ne "0" ]; then
            echo "$txres"
            exit
        fi
        txhash=$(echo "$txres" | awk '/txhash:/ {print $2}')
    else
        echo "$txres"
        exit
    fi

    local x=1
    while [ $x -le 10 ]
    do
        if result=$($CMD q tx $txhash 2>&1); then
            echo "$result"
            return
        fi
        sleep 1
        x=$(( $x + 1 ))        
    done
}