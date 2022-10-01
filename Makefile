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

export ALICE_LOCK = 04b4ac68eff3a82d86db5f0489d66f91707e99943bf796ae6a2dcb2205c9522fa7915428b5ac3d3b9291e62142e7246d85ad54504fabbdb2bae5795161f8ddf259
export ALICE_SECRET = 3c9229289a6125f7fdf1885a77bb12c37a8d3b4962d936f7e3084dece32a3ca1   
export BOB_LOCK = 042d5f7beb52d336163483804facb17c47033fb14dfc3f3c88235141bae1896fc8d99a685aafaf92d5f41d866fe387b988a998590326f1b549878b9d03eabed7e5
export BOB_SECRET= cde73ee8f8584c54ac455c941f75990f4bff47a4340023e3fd236344e9a7d4ea  

########################
## JUNO UNI-3 Testnet ##
########################
# export NODE = https://rpc.uni.juno.deuslabs.fi:443
# export CHAINID = uni-3
# export FEETOKEN = ujunox
# export NRIDE = juno1caapzpyuhddkzps9nwatyknlvmm2av6whkk7aqse4umzmp0gpm5se7nzg7 # address of the cw20 token smart-contract
# export ESCROW = juno1eds9t7rpfsfeyu35nevyc8tglumvejjg6p0yegkrv4wjlf0lghtqwy75uv # address of the escrow smart-contract

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
	./scripts/token-send.sh $(from) $(to)

token-send-grant:
	./scripts/token-send-grant.sh $(from) $(to)

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



