export CMD = junod

# Uncomment the section for required network

###########
## LOCAL ##
###########
export NODE = http://localhost:26657
export CHAINID = testing
export FEETOKEN = ujunox
export NRIDE = juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8
export ESCROW = juno1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrq68ev2p

########################
## JUNO UNI-3 Testnet ##
########################
# export NODE = https://rpc.uni.juno.deuslabs.fi:443
# export CHAINID = uni-3
# export FEETOKEN = ujunox
# export NRIDE = juno1caapzpyuhddkzps9nwatyknlvmm2av6whkk7aqse4umzmp0gpm5se7nzg7 # address of the cw20 token smart-contract
# export ESCROW = juno1eds9t7rpfsfeyu35nevyc8tglumvejjg6p0yegkrv4wjlf0lghtqwy75uv # address of the escrow smart-contract

start-node:
	./scripts/docker-run.sh

stop-node:
	./scripts/docker-stop.sh

compile-cw20:
	./scripts/compile-workspace.sh "$(shell pwd)/cw-plus"

compile-i4i:
	./scripts/compile.sh "$(shell pwd)/cw-i4i"

deploy-cw20:
	./scripts/deploy-cw20.sh

init-cw20:
	./scripts/init-cw20.sh $(code)

deploy-escrow:
	./scripts/deploy-escrow.sh
	
init-escrow:
	./scripts/init-escrow.sh $(code)

token-info:
	./scripts/token-info.sh

token-balance-list:
	./scripts/token-balance-list.sh

token-balance:
	./scripts/token-balance.sh $(addr)

token-send:
	./scripts/token-send.sh $(from) $(to)

token-send-grant:
	./scripts/token-send-grant.sh $(from) $(to)

escrow-create:
	./scripts/escrow-create.sh "test-escrow" $(b) "lock" 

escrow-list:
	./scripts/escrow-list.sh

escrow-details:
	./scripts/escrow-details.sh

escrow-approve:
	./scripts/escrow-approve.sh

feegrant-list:
	./scripts/feegrant-list.sh $(acc)

feegrant-create:
	./scripts/feegrant-create.sh $(acc)

feegrant-revoke:
	./scripts/feegrant-revoke.sh $(acc)

native-balance:
	./scripts/native-balance.sh $(addr)

native-send:
	./scripts/native-send.sh $(to)