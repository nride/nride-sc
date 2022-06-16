
# NRIDE Smart-Contracts

This repo contains the set of smart-contracts that support nRide's token infrastructure

## CW20-Base

NRIDE is implemented as a cw20 token for CosmWasm platforms

### Dev Environment

#### Run a node, deploy and initialize the token contract:

```
make start
```

#### Check balances

```
junod query wasm contract-state smart juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 \
'{"balance":{"address":"juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y"}}' 
``` 

```
junod query wasm contract-state smart juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 \
'{"balance":{"address":"juno1zzlldpx4l5t6zvz8tg9tyz2hpmnyunde2uq76k"}}' 
```

#### Send

From 'tester' to 'alice'

```
junod tx wasm execute juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 \
'{"transfer":{"amount":"200","recipient":"juno1zzlldpx4l5t6zvz8tg9tyz2hpmnyunde2uq76k"}}' \
--from tester \
--chain-id testing \
-b block \
-y
```

#### Stop and delete everything

```
make stop
```
