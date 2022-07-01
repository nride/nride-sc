# JUNO Testnet (UNI)

## Account

name (on my machine): `faucet`
mnemonic: cabbage enroll purchase sell arm extra awful leg prison snow sure welcome shoe matrix office black much mask physical photo giant pattern enable depend
address: juno1hpxxsgxmrqxm6rsk2ghzdlplvzr48cfwhn0zt0


name: `alice`
mnemonic: engage capital accident mimic fork vehicle grocery student glad pear rotate sausage scrub annual buffalo debris game miracle evolve baby wagon apple flush coffee
address: juno1agpgyrrvesdu62dmurgdnjt2uvjmflau6mlzqq

## Example Commands

```
junod query bank balances juno1hpxxsgxmrqxm6rsk2ghzdlplvzr48cfwhn0zt0 --node https://rpc.uni.juno.deuslabs.fi:443
```

```
junod tx wasm store contracts/cw20_base.wasm  \
    --from martin-testnet \
    --chain-id=uni-3 \
    --gas-prices 0.1ujunox \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y \
    --node https://rpc.uni.juno.deuslabs.fi:443
```


==> `code id = 407`

```
junod tx wasm instantiate 407 \
    '{"name":"NRIDE Coin","symbol":"NRIDE","decimals":6,"initial_balances":[{"address":"juno1hpxxsgxmrqxm6rsk2ghzdlplvzr48cfwhn0zt0","amount":"1000000"}]}' \
    --amount 50000ujunox  \
    --label "NRIDE TOKEN INIT BALANCES" \
    --admin juno1hpxxsgxmrqxm6rsk2ghzdlplvzr48cfwhn0zt0 \
    --from martin-testnet \
    --chain-id uni-3 \
    --gas-prices 0.1ujunox \
    --gas auto \
    --gas-adjustment 1.3 \
    -b block \
    -y \
   --node https://rpc.uni.juno.deuslabs.fi:443
```

==> `contract address = juno1caapzpyuhddkzps9nwatyknlvmm2av6whkk7aqse4umzmp0gpm5se7nzg7`

## Check Balance

```
junod query wasm contract-state smart juno1caapzpyuhddkzps9nwatyknlvmm2av6whkk7aqse4umzmp0gpm5se7nzg7 \
'{"balance":{"address":"juno1hpxxsgxmrqxm6rsk2ghzdlplvzr48cfwhn0zt0"}}' \
--node https://rpc.uni.juno.deuslabs.fi:443
```

## Send 

```
junod tx wasm execute juno1caapzpyuhddkzps9nwatyknlvmm2av6whkk7aqse4umzmp0gpm5se7nzg7 \
'{"transfer":{"amount":"200","recipient":"juno1agpgyrrvesdu62dmurgdnjt2uvjmflau6mlzqq"}}' \
--from faucet \
--fees "5000ujunox" \
--chain-id uni-3 \
-b block \
-y \
--node https://rpc.uni.juno.deuslabs.fi:443
```
## Check Alice balance

```
junod query wasm contract-state smart juno1caapzpyuhddkzps9nwatyknlvmm2av6whkk7aqse4umzmp0gpm5se7nzg7 \
'{"balance":{"address":"juno1agpgyrrvesdu62dmurgdnjt2uvjmflau6mlzqq"}}' \
--node https://rpc.uni.juno.deuslabs.fi:443
```
