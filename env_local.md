# Local Dev Environment (Docker)

## Run a node, deploy and initialize the token contract:

```
make start
make deploy-cw20
```

## Check balances

```
junod query wasm contract-state smart juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 \
'{"balance":{"address":"'"$(junod keys show -a faucet)"'"}}'
``` 

```
junod query wasm contract-state smart juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 \
'{"balance":{"address":"'"$(junod keys show -a alice)"'"}}'
``` 

## Send

From 'faucet' to 'alice'

```
junod tx wasm execute juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 \
'{"transfer":{"amount":"200","recipient":"'"$(junod keys show -a alice)"'"}}' \
--from faucet \
--chain-id testing \
-b block \
-y
```

## Stop and delete everything

```
make stop
```

