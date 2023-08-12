#!/bin/sh

function execute_tx_block() {
    local command=$1
    
    echo $1
    
    if txres=$($command 2>&1); then
        txcode=$(echo "$txres" | awk '/code:/ {print $2}')
        if [ "$txcode" -ne "0" ]; then
            echo "$txres"
            exit
        fi
        txhash=$(echo "$txres" | awk '/txhash:/ {print $2}')

        echo "AAA $txres"
    else
        echo "XXX $txres"
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

    echo "ZZZ $result"
}

function execute_tx_block_2() {
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
            exit
        fi
        sleep 1
        x=$(( $x + 1 ))        
    done
}

function execute_wrapper() {
    echo $#
    res=$($1)
    # if txres=$($command 2>&1); then
    #     echo "OK $txres"
    # else
    #     echo "ERROR $txres"
    # fi
}

function execute_array() {
    echo "args: $#"
    
    a=("$@")
    
    echo "0: ${a[0]}"
    echo "1: ${a[1]}"
    echo "2: ${a[2]}"
    
    res=$(${a[0]} "${a[2]}")

    echo "$res"

    ((last_idx=${#a[@]} - 1))

    cmd="${a[0]}"

    args=()
    for i in `seq 1 $last_idx` ; do
        args+=("${a[$i]}")
    done

    if txres=$($cmd "${args[@]}" 2>&1); then
        echo "OK $txres"
    else
        echo "ERROR $txres"
    fi

    # 
    # a=("$@")

    # res=$($a)

    # echo $res
}