
# NRIDE Smart-Contracts

This repo contains the set of smart-contracts that support nRide's token infrastructure

## Requirements

1) install the junod CLI: https://docs.junonetwork.io/validators/getting-setup

(install v10.0.2 or above)

2) create keys called `faucet`, `alice` and `bob` using junod:

```
junod keys add faucet
junod keys add alice
junod keys add bob
```

These account names are used in the scripts to test sending and receiving tokens.

3) (For testnet environment only) request testnet tokens for the `faucet` account by writing a message in Juno's faucet channel: https://discord.com/channels/816256689078403103/842073995059003422

## Deployment

We currently support two environments "Local" and "Testnet".

=> Comment out the relevant section in the Makefile <=

### (Optional) Compile Contracts

ATTENTION: This can take a long time

```
make compile-cw20
make compile-i4i
```

### Local Env (Docker)

1) Run a local `junod` node with `make start-node` (requires docker)
2) Deploy and initialise cw20 contract: 
```
make deploy-cw20
make init-cw20 code=1
```
3) Deploy and initialize escrow contract (i4i):
```
make deploy-i4i
make init-i4i code=2
```

### Testnet Env (JUNO UNI-3 Testnet)

1) Deploy and initialize cw20 contract:

```
make deploy-cw20
# make note of resulting code-id
make init-cw20 code=[code-id]
```

make note of the resulting contract address and copy it in the Makefile
under the NRIDE variable

2) Same thing with escrow contract:

```
make deploy-i4i
# make note of resulting code-id
make init-i4i code=[code-id]
```

make note of the resulting contract address and copy it in the Makefile
under the ESCROW variable

## Usage

With the environment setup, we can run some commands to interract with the smart-contracts.

For example to send nride tokens from `faucet` to `alice`:

```
make token-send from=faucet to=alice
```

There are many other Makefile recipes to test...

## Smart Contracts

### cw20_base

This repo contains a git submodule under `cw-plus` containing a set of "official" smart-contracts,
include the cw20_base which are using to implement our token.

### cw_i4i

Our escrow smart-contract based on cw20_escrow: https://github.com/CosmWasm/cw-tokens/tree/main/contracts/cw20-escrow