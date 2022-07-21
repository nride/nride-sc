#!/bin/sh

docker kill juno_node_1 && docker rm juno_node_1

echo '@@@ juno_node_1 killed and removed'