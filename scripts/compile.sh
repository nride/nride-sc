#!/bin/sh

set -eux

CON_DIR=$1
CACHE_DIR=$CON_DIR"_cache"

docker run --rm -v "$CON_DIR":/code \
  --mount type=volume,source="$(basename "$CACHE_DIR")",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.14.0