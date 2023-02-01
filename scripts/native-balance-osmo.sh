#!/bin/sh

ACC=$1

osmosisd q bank balances $(osmosisd keys show -a $ACC) --node $OSMONODE