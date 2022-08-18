
# NRIDE Smart-Contracts

This repo contains the set of smart-contracts that support nRide's token infrastructure

## Requirements

1) install the junod CLI: https://docs.junonetwork.io/validators/getting-setup

2) create keys called `faucet`, `alice` and `bob` using junod:

```
junod keys add faucet
junod keys add alice
junod keys add bob
```

These account names are used in the scripts to test sending and receiving tokens.

3) request testnet tokens for the `faucet` account by writing a message in Juno's faucet channel: https://discord.com/channels/816256689078403103/842073995059003422

## Usage

If you wish to re-deploy the smart-contracts, run `make deploy-cw20` and `make deploy-escrow`, take note of the contract addresses and replace them in the Makefile, in place of the `NRIDE` and `ESCROW` variables.

Otherwise, the token and escrow contracts are already deployed on the Juno testnet so you can interact with them directly.

For example to send nride tokens from `faucet` to `alice`:


```
make token-send from=faucet to=alice
```

There are many other Makefile recipes to test...

## Smart Contracts

cw20_base: https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw20-base

cw20_escrow: https://github.com/CosmWasm/cw-tokens/tree/main/contracts/cw20-escrow