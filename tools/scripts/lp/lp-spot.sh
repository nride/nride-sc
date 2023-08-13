#!/bin/sh

POOL=$1
QUOTE_ASSET=$2
BASE_ASSET=$3

osmosisd q gamm spot-price $POOL $QUOTE_ASSET $BASE_ASSET --node $OSMONODE