

## Start node

```
docker run -it \
--name juno_node_1 \
-p 1317:1317 \
-p 26656:26656 \
-p 26657:26657 \
-e STAKE_TOKEN=ujunox \
-e UNSAFE_CORS=true \
ghcr.io/cosmoscontracts/juno:v5.0.1 \
./setup_and_run.sh juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y
```

## Deploy cw20_base contract

```
junod tx wasm store cw20_base.wasm  --from tester --chain-id=testing --gas-prices 0.1ujunox --gas auto --gas-adjustment 1.3 -b block -y
```

## Init contract

```
junod tx wasm instantiate 1 \
'{"name":"NRIDE Coin","symbol":"NRIDE","decimals":6,"initial_balances":[{"address":"juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y","amount":"12345678000"}]}' \
--amount 50000ujunox  \
--label "NRIDE TOKEN INIT BALANCES" \
--no-admin \
--from tester \
--chain-id testing \
--gas-prices 0.1ujunox \
--gas auto \
--gas-adjustment 1.3 \
-b block \
-y    
```

// contract address: 
juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8

```
junod query wasm contract juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8
```

## Query contract state

```
junod query wasm contract-state smart juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 \
'{"balance":{"address":"juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y"}}' 
``` 

## Alice Account

juno1zzlldpx4l5t6zvz8tg9tyz2hpmnyunde2uq76k

## Send

```
junod tx wasm execute juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 \
'{"transfer":{"amount":"200","recipient":"juno1zzlldpx4l5t6zvz8tg9tyz2hpmnyunde2uq76k"}}' \
--from tester \
--chain-id testing \
-b block \
-y
```

```
junod query wasm contract-state smart juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 \
'{"balance":{"address":"juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y"}}' 
``` 

```
junod query wasm contract-state smart juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 \
'{"balance":{"address":"juno1zzlldpx4l5t6zvz8tg9tyz2hpmnyunde2uq76k"}}' 
```