export CMD = junod
export NODE = https://rpc.uni.juno.deuslabs.fi:443
export CHAINID = uni-3
export FEETOKEN = ujunox
export NRIDE = juno1caapzpyuhddkzps9nwatyknlvmm2av6whkk7aqse4umzmp0gpm5se7nzg7
export ESCROW = juno1eds9t7rpfsfeyu35nevyc8tglumvejjg6p0yegkrv4wjlf0lghtqwy75uv

deploy-cw20:
	./scripts/deploy-cw20.sh
	./scripts/init-cw20.sh

deploy-escrow:
	./scripts/deploy-escrow.sh
	./scripts/init-escrow.sh

token-info:
	./scripts/token-info.sh

token-balance-list:
	./scripts/token-balance-list.sh

token-balance:
	./scripts/token-balance.sh $(addr)

token-send:
	./scripts/token-send.sh $(from) $(to)

escrow-create:
	./scripts/escrow-create.sh

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