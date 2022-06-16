#!/bin/sh

docker run -d \
    --name juno_node_1 \
    -p 1317:1317 \
    -p 26656:26656 \
    -p 26657:26657 \
    -e STAKE_TOKEN=ujunox \
    -e UNSAFE_CORS=true \
    ghcr.io/cosmoscontracts/juno:v5.0.1 \
    ./setup_and_run.sh juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y

echo '@@@ juno_node_1 started in background'