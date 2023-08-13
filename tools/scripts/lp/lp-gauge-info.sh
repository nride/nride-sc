#!/bin/sh

ID=$1

osmosisd q incentives gauge-by-id $ID --node $OSMONODE
