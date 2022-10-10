#!/bin/sh

docker run -d \
    --name juno_node_1 \
    -p 1317:1317 \
    -p 26656:26656 \
    -p 26657:26657 \
    -p 9090:9090 \
    -e STAKE_TOKEN=ujunox \
    -e UNSAFE_CORS=true \
    juno:v10.0.0-alpha.2 \
    ./setup_and_run.sh $(junod keys show -a faucet)

sleep 5

# ghcr.io/cosmoscontracts/juno:v10.0.2 \

echo '@@@ juno_node_1 started in background'