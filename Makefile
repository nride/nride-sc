export CMD = junod

# Uncomment the section for required network

###########
## LOCAL ##
###########
export NODE = http://localhost:26657
export CHAINID = testing
export FEETOKEN = ujunox
export NATIVE = ucosm
export NRIDE = juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8
export ESCROW = juno1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrq68ev2p
export REGISTRY = juno17p9rzwnnfxcjp32un9ug7yhhzgtkhvl9jfksztgw5uh69wac2pgszu8fr9

########################
## JUNO Mainnet       ##
########################
# export NODE = https://rpc-juno.mib.tech:443
# export CHAINID = juno-1
# export FEETOKEN = ujuno
# # NATIVE is the denom of the native token used for native payments (axlUSDC)
# export NATIVE = ibc/EAC38D55372F38F1AFD68DF7FE9EF762DCF69F26520643CF3F9D292A738D8034
# # NRIDE is the address of the NRIDE CW20 contract
# export NRIDE = juno1qmlchtmjpvu0cr7u0tad2pq8838h6farrrjzp39eqa9xswg7teussrswlq 
# # code_id: 4254
# export ESCROW = juno1lgymnx0lhhz09pdknpmllfjvkkt80vsyyqxck6wqu9qel5uk2axqh0j00y
# # OLD ---
# # ESCROW is the address of the nRide escrow contract
# # code_id: 3638
# # export ESCROW = juno10lmjjk2uj7cehk2972245ul0x36hv5m29wsacm0mmx69dkg78lgs7kh96z
# # ---
# # REGISTRY is the addres of the nRide registry contract
# export REGISTRY = juno19385sh6ze678s6x49grc08cyqqmkv7djhy3wxqy6hnwc0ykkx9psa74hvc

################
## Demo Locks ##
################ 
export ALICE_LOCK = 0330347c5cb0f1627bdd2e7b082504a443b2bf50ad2e3efbb4e754ebd687c78c24
export ALICE_SECRET = 27874aa2b70ce7281c94413c36d44fac6fa6a1198f2c529188c4dd4f7a4e1870

#############################

compile-cw20:
	./scripts/compile-workspace.sh "$(shell pwd)/cw-plus"

compile-escrow:
	./scripts/compile.sh "$(shell pwd)/cw-nride-escrow"

compile-registry:
	./scripts/compile.sh "$(shell pwd)/cw-nride-registry"

#############################

start-node:
	./scripts/docker-run.sh

stop-node:
	./scripts/docker-stop.sh

#############################

# deploy and intialize the contracts
# create grants for alice and bob accounts so that they can submit tx to the blockchain
# give some NRIDE tokens to alice and bob
demo-bootstrap:
	./scripts/bootstrap.sh

demo-create-cw20:
	./scripts/escrow/escrow-create-cw20.sh alice test bob $(ALICE_LOCK)

demo-create-native:
	./scripts/escrow/escrow-create-native.sh alice test bob 100 $(NATIVE) $(ALICE_LOCK)

demo-withdraw:
	./scripts/escrow/escrow-withdraw.sh bob test $(ALICE_SECRET)

demo-cancel:
	./scripts/escrow/escrow-cancel.sh alice test

demo-details:
	./scripts/escrow/escrow-details.sh test

#############################

feegrant-list:
	./scripts/feegrant/feegrant-list.sh $(grantee)

feegrant-create:
	./scripts/feegrant/feegrant-create.sh $(grantee)

feegrant-revoke:
	./scripts/feegrant/feegrant-revoke.sh $(grantee)

################################

native-balance:
	./scripts/native-token/native-balance.sh $(addr)

native-balance-list:
	./scripts/native-token/native-balance-list.sh

native-send:
	./scripts/native-token/native-send.sh $(to) $(amount) $(denom)

################################

deploy-cw20:
	./scripts/nride-token/deploy-cw20.sh

init-cw20:
	./scripts/nride-token/init-cw20.sh $(code)

token-info:
	./scripts/nride-token/token-info.sh

token-balance-list:
	./scripts/nride-token/token-balance-list.sh

token-balance:
	./scripts/nride-token/token-balance.sh $(addr)

token-send:
	./scripts/nride-token/token-send.sh $(from) $(to) $(amount)

token-send-to:
	./scripts/nride-token/token-send-to.sh $(from) $(to-addr) $(amount)

token-send-grant:
	./scripts/nride-token/token-send-grant.sh $(from) $(to) $(amount)

################################

deploy-escrow:
	./scripts/escrow/deploy-escrow.sh
	
init-escrow:
	./scripts/escrow/init-escrow.sh $(code)

escrow-create-cw20:
	./scripts/escrow/escrow-create-cw20.sh $(from) $(id) $(userb) $(ALICE_LOCK) 

escrow-create-native:
	./scripts/escrow/escrow-create-native.sh $(from) $(id) $(userb) $(amount) $(denom) $(ALICE_LOCK) 

escrow-cancel:
	./scripts/escrow/escrow-cancel.sh $(from) $(id)

escrow-withdraw:
	./scripts/escrow/escrow-withdraw.sh $(from) $(id) $(ALICE_SECRET)

escrow-list:
	./scripts/escrow/escrow-list.sh

escrow-details:
	./scripts/escrow/escrow-details.sh $(id)

###################################

deploy-registry:
	./scripts/registry/deploy-registry.sh

init-registry:
	./scripts/registry/init-registry.sh $(code)

migrate-registry:
	./scripts/registry/migrate-registry.sh $(REGISTRY) $(code)

registry-subscribe:
	./scripts/registry/registry-subscribe.sh $(from) $(nkn) $(location) 

registry-details:
	./scripts/registry/registry-details.sh $(addr)

# ex: make registry-list location=paris
registry-list:
	./scripts/registry/registry-list.sh $(location)

# ex: make registry-list-multiple locations=paris,london
registry-list-multiple:
	./scripts/registry/registry-list-multiple.sh $(locations)
