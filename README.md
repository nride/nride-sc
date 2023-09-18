
# NRIDE Smart-Contracts

This repo contains the set of smart-contracts that are used by nRide. This 
includes the cw20 token, escrow, and driver registry.

## CW20 Token

The nRide token is implemented as a `CW20` Token.

This repo uses a git submodule to track the official specs and implementations 
of the CW20 standard, as well as its dependencies, in the upstream repo 
`cw-plus`.

To compile:

```sh
make compile-cw20
```

## Escrow 

The escrow smart-contract implements an mechanism used for paying for nRide 
journeys.

To compile:

```sh
make compile-escrow
```

More info about the escrow in [cw-nride-escrow](cw-nride-escrow/README.md)

## Demo

We have two users, Alice and Bob, who will be using the escrow contract. Alice 
creates the escrow, locking 100 NRIDE tokens, with Bob as the counterparty. At 
any time, Alice can call the `Cancel` method to cancel the escrow and get her
deposit back. To withdraw from the escrow, Bob will need to obtain the secret 
key from `Alice`. This happens off-chain (by sending a private message for 
example).

Here, we will walk through the happy case, where Bob obtains the secret key from
Alice and withdraws from the escrow, but we encourage the reader to try 
withdrawing with the wrong key, or to cancel the escrow from Alice's account.

Note: This has only been tested on a Macbook Air with M1 processor

### Prerequisites

1) install the junod CLI (v10.0.2 or above): 
https://docs.junonetwork.io/validators/getting-setup#build-juno-from-source

2) create keys called `faucet`, `alice` and `bob` using junod:

```
junod keys add faucet
junod keys add alice
junod keys add bob
```

These account names are used in the scripts to test sending and receiving tokens.

3) (For testnet environment only) request testnet tokens for the `faucet` 
account by writing a message in Juno's faucet channel: 
https://discord.com/channels/816256689078403103/842073995059003422

4) Install Docker

5) If you haven't done so already, compile the three contracts.

Note: this can take a long time

```
make compile-cw20
make compile-escrow
make compile-registry
```

### Run the local Juno node

```
make start-node
```

### Initialize the smart-contracts and the demo environment (fund accounts etc)

```
make demo-bootstrap
```

### Test happy case

Note: 
> at any time in this demo, run `make demo-details` and `make token-balalance-list`
> to see what is happening with the escrow and with the user balances.


1) Alice creates an escrow with Bob as counterparty

```
make demo-create
```

5) Bob withdraws

```
make demo-withdraw
```

6) See escrow status and balances

```
make demo-details
make token-balance-list
```

### Have fun!

Try using `make demo-cancel` to see what happens if Alice cancels. Or try 
withdrawing with the wrong secret.

## Registry

The registry smart-contract implements a database of drivers with their NKN 
address and location.

To compile:

```
make compile-registry
```

More info about the escrow in [cw-nride-registry](cw-nride-registry/README.md)
