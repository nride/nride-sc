#!/bin/sh

__dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

source ${__dir}/nride-token/deploy-cw20.sh
source ${__dir}/nride-token/init-cw20.sh 1

source ${__dir}/escrow/deploy-escrow.sh
source ${__dir}/escrow/init-escrow.sh 2

source ${__dir}/registry/deploy-registry.sh
source ${__dir}/registry/init-registry.sh 3

source ${__dir}/feegrant/feegrant-create.sh alice
source ${__dir}/feegrant/feegrant-create.sh bob

source ${__dir}/nride-token/token-send.sh faucet alice 1000
source ${__dir}/nride-token/token-send.sh faucet bob 1000
source ${__dir}/native-token/native-send.sh alice 1000 $NATIVE
source ${__dir}/native-token/native-send.sh bob 1000 $NATIVE
