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
# export REGISTRY = juno17p9rzwnnfxcjp32un9ug7yhhzgtkhvl9jfksztgw5uh69wac2pgszu8fr9

########################
## JUNO Mainnet       ##
########################
export NODE = https://rpc-juno.mib.tech:443
export CHAINID = juno-1
export FEETOKEN = ujuno
# NRIDE is the address of the NRIDE CW20 contract
export NRIDE = juno1qmlchtmjpvu0cr7u0tad2pq8838h6farrrjzp39eqa9xswg7teussrswlq 
# ESCROW is the address of the nRide escrow contract
export ESCROW = juno18a4v5qekm9a5dugfhkgyzv6wcxlys0jwh9p4y7k0hfuf9xvxhaksme8fac
# REGISTRY is the addres of the nRide registry contract
export REGISTRY = juno19385sh6ze678s6x49grc08cyqqmkv7djhy3wxqy6hnwc0ykkx9psa74hvc
# ICS20 is the address of the CW20-ICS20 contract that allows sending CW2O tokens
# through IBC
export ICS20 = juno1v4887y83d6g28puzvt8cl0f3cdhd3y6y9mpysnsp3k8krdm7l6jqgm0rkn
# JUNO_CHANNEL is the channel from JUNO to OSMOSIS, linked to the CW20-ICS20 
# contract above, through which NRIDE tokens are sent to OSMOSIS
export JUNO_CHANNEL = channel-47

########################
## OSMOSIS Mainnet    ##
########################
export OSMONODE = https://osmosis-rpc.polkachu.com:443
export OSMO_CHANNEL = channel-169
# IBCDENOM is the denomination of NRIDE tokens on OSMOSIS sent throught the 
# CW20-ICS20 channel configured above
export IBCDENOM = ibc/E750D31033DC1CF4A044C3AA0A8117401316DC918FBEBC4E3D34F91B09D5F54C
# OSMO_POOL is the ID of the NRIDE/OSMO pool on Osmosis
export OSMO_POOL = 924
# GAUGE_ID is the ID of the incentives gauge in the osmo-nride pool
export GAUGE_ID =  32110

#############################

native-balance-osmo:
	./scripts/native-balance-osmo.sh $(acc)

#############################

ics20-channels:
	./scripts/ibc/ics20-channels.sh

ics20-channel:
	./scripts/ibc/ics20-channel.sh $(JUNO_CHANNEL)

ics20-transfer:
	./scripts/ibc/ics20-transfer.sh $(from) $(to) $(amount) $(JUNO_CHANNEL)

ics20-redeem:
	./scripts/ibc/ics20-redeem.sh $(from) $(to) $(amount) $(OSMO_CHANNEL)

#############################

lp-create:
	./scripts/lp/lp-create.sh $(config) $(acc)

lp-info:
	./scripts/lp/lp-info.sh $(OSMO_POOL)

lp-spot:
	./scripts/lp/lp-spot.sh $(OSMO_POOL) uosmo $(IBCDENOM)

lp-liquidity:
	./scripts/lp/lp-liquidity.sh $(OSMO_POOL)

lp-share:
	./scripts/lp/lp-share.sh $(OSMO_POOL)

lp-gauge-info:
	./scripts/lp/lp-gauge-info.sh $(GAUGE_ID)

lp-gauge-add:
	./scripts/lp/lp-gauge-add.sh $(acc) $(amount) $(GAUGE_ID)

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
