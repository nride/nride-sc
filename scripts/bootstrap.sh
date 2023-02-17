#!/bin/sh

__dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

source ${__dir}/deploy-cw20.sh
source ${__dir}/init-cw20.sh 1

source ${__dir}/deploy-escrow.sh
source ${__dir}/init-escrow.sh 2

source ${_dir}/deploy-registry.sh
source ${_dir}/init-registry.sh 3

source ${__dir}/feegrant-create.sh alice
source ${__dir}/feegrant-create.sh bob

source ${__dir}/token-send.sh faucet alice 1000
source ${__dir}/token-send.sh faucet bob 1000
