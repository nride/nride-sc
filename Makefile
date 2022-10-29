export CMD = junod

# Uncomment the section for required network

###########
## LOCAL ##
###########
# export NODE = http://localhost:26657
# export CHAINID = testing
# export FEETOKEN = ujunox
# export NRIDE = juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8
# export ESCROW = juno1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrq68ev2p

########################
## JUNO UNI-5 Testnet ##
########################
export NODE = https://rpc.uni.junonetwork.io:443
export CHAINID = uni-5
export FEETOKEN = ujunox
export NRIDE = juno1q9wr0p5wklvjusgaeq95fhfhg7mmtjn66z0aeky4acfnd2v62qhsdsd4nl
export ESCROW = juno1pyqazts4um8je4ard7s52y3nhfqaayl94jzmtwxzjnqj2jt3ehrq4avmjk # codeid 2736

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

compile-i4i:
	./scripts/compile.sh "$(shell pwd)/cw-i4i"

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
	./scripts/escrow-create.sh alice test bob $(ALICE_LOCK)

demo-topup:
	./scripts/escrow-topup.sh bob test $(BOB_LOCK)

demo-approve-alice:
	./scripts/escrow-approve.sh alice test $(BOB_SECRET)

demo-approve-bob:
	./scripts/escrow-approve.sh bob test $(ALICE_SECRET)

demo-cancel-alice:
	./scripts/escrow-cancel.sh alice test

demo-cancel-bob:
	./scripts/escrow-cancel.sh bob test

demo-withdraw:
	./scripts/escrow-withdraw.sh bob test

demo-details:
	./scripts/escrow-details.sh test

#############################

escrow-create:
	./scripts/escrow-create.sh $(from) $(id) $(userb) $(ALICE_LOCK) 

escrow-topup:
	./scripts/escrow-topup.sh $(from) $(id) $(BOB_LOCK)

escrow-approve:
	./scripts/escrow-approve.sh $(from) $(id) $(secret)

escrow-cancel:
	./scripts/escrow-cancel.sh $(from) $(id)

escrow-withdraw:
	./scripts/escrow-withdraw.sh $(from) $(id)

escrow-list:
	./scripts/escrow-list.sh

escrow-details:
	./scripts/escrow-details.sh $(id)

#############################

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
	./scripts/token-send.sh $(from) $(to) $(amount)

token-send-grant:
	./scripts/token-send-grant.sh $(from) $(to) $(amount)

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

#############################



