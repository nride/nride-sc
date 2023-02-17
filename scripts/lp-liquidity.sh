#!/bin/sh

POOL=$1

osmosisd q gamm total-pool-liquidity $POOL --node $OSMONODE