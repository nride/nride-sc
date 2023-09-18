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
export REGISTRY = juno17p9rzwnnfxcjp32un9ug7yhhzgtkhvl9jfksztgw5uh69wac2pgszu8fr9

########################
## JUNO Mainnet       ##
########################
# export NODE = https://rpc-juno.mib.tech:443
# export CHAINID = juno-1
# export FEETOKEN = ujuno
# # NRIDE is the address of the NRIDE CW20 contract
# export NRIDE = juno1qmlchtmjpvu0cr7u0tad2pq8838h6farrrjzp39eqa9xswg7teussrswlq 
# # ESCROW is the address of the nRide escrow contract
# export ESCROW = juno18a4v5qekm9a5dugfhkgyzv6wcxlys0jwh9p4y7k0hfuf9xvxhaksme8fac
# # REGISTRY is the addres of the nRide registry contract
# export REGISTRY = juno19385sh6ze678s6x49grc08cyqqmkv7djhy3wxqy6hnwc0ykkx9psa74hvc

################
## Demo Locks ##
################ 
export ALICE_LOCK = 0330347c5cb0f1627bdd2e7b082504a443b2bf50ad2e3efbb4e754ebd687c78c24
export ALICE_SECRET = 27874aa2b70ce7281c94413c36d44fac6fa6a1198f2c529188c4dd4f7a4e1870

export BOB_LOCK = 032d5f7beb52d336163483804facb17c47033fb14dfc3f3c88235141bae1896fc8
export BOB_SECRET= cde73ee8f8584c54ac455c941f75990f4bff47a4340023e3fd236344e9a7d4ea

#############################

compile-cw20:
	./scripts/compile-workspace.sh "$(shell pwd)/cw-plus"

compile-escrow:
	./scripts/compile.sh "$(shell pwd)/cw-nride-escrow"

compile-escrow-v2:
	./scripts/compile.sh "$(shell pwd)/cw-nride-escrow-v2"

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

demo-create:
	./scripts/escrow/escrow-create.sh alice test bob $(ALICE_LOCK)

demo-topup:
	./scripts/escrow/escrow-topup.sh bob test $(BOB_LOCK)

demo-approve-alice:
	./scripts/escrow/escrow-approve.sh alice test $(BOB_SECRET)

demo-approve-bob:
	./scripts/escrow/escrow-approve.sh bob test $(ALICE_SECRET)

demo-cancel-alice:
	./scripts/escrow/escrow-cancel.sh alice test

demo-cancel-bob:
	./scripts/escrow/escrow-cancel.sh bob test

demo-withdraw:
	./scripts/escrow/escrow-withdraw.sh bob test

demo-details:
	./scripts/escrow/escrow-details.sh test

#############################

# deploy and intialize the contracts
# create grants for alice and bob accounts so that they can submit tx to the blockchain
# give some NRIDE tokens to alice and bob
demo-v2-bootstrap:
	./scripts/bootstrap-v2.sh

demo-v2-create:
	./scripts/escrow-v2/escrow-create.sh alice test bob $(ALICE_LOCK)

demo-v2-withdraw:
	./scripts/escrow-v2/escrow-withdraw.sh bob test $(ALICE_SECRET)

demo-v2-cancel:
	./scripts/escrow-v2/escrow-cancel.sh alice test

demo-v2-details:
	./scripts/escrow-v2/escrow-details.sh test

#############################

escrow-create:
	./scripts/escrow/escrow-create.sh $(from) $(id) $(userb) $(ALICE_LOCK) 

escrow-topup:
	./scripts/escrow/escrow-topup.sh $(from) $(id) $(BOB_LOCK)

escrow-approve:
	./scripts/escrow/escrow-approve.sh $(from) $(id) $(secret)

escrow-cancel:
	./scripts/escrow/escrow-cancel.sh $(from) $(id)

escrow-withdraw:
	./scripts/escrow/escrow-withdraw.sh $(from) $(id)

escrow-list:
	./scripts/escrow/escrow-list.sh

escrow-details:
	./scripts/escrow/escrow-details.sh $(id)

#############################

escrow-v2-create:
	./scripts/escrow-v2/escrow-create.sh $(from) $(id) $(userb) $(ALICE_LOCK) 

escrow-v2-cancel:
	./scripts/escrow-v2/escrow-cancel.sh $(from) $(id)

escrow-v2-withdraw:
	./scripts/escrow-v2/escrow-withdraw.sh $(from) $(id) $(ALICE_SECRET)

escrow-v2-list:
	./scripts/escrow-v2/escrow-list.sh

escrow-v2-details:
	./scripts/escrow-v2/escrow-details.sh $(id)

#############################

deploy-cw20:
	./scripts/nride-token/deploy-cw20.sh

init-cw20:
	./scripts/nride-token/init-cw20.sh $(code)

deploy-escrow:
	./scripts/escrow/deploy-escrow.sh
	
init-escrow:
	./scripts/escrow/init-escrow.sh $(code)

deploy-escrow-v2:
	./scripts/escrow-v2/deploy-escrow.sh

init-escrow-v2:
	./scripts/escrow-v2/init-escrow.sh $(code)

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

feegrant-list:
	./scripts/feegrant/feegrant-list.sh $(grantee)

feegrant-create:
	./scripts/feegrant/feegrant-create.sh $(grantee)

feegrant-revoke:
	./scripts/feegrant/feegrant-revoke.sh $(grantee)

native-balance:
	./scripts/native-token/native-balance.sh $(addr)

native-send:
	./scripts/native-token/native-send.sh $(to) $(amount)

################################

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
