#!/bin/sh

function execute_tx_block() {
    local command=$1
    
    if txres=$($command 2>&1); then
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
            exit
        fi
        sleep 1
        x=$(( $x + 1 ))        
    done

    echo "$result"
}