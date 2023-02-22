
# NRIDE Smart-Contracts

This repo contains the set of smart-contracts that support nRide's token 
infrastructure

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

The escrow smart-contract implements an advanced escrow mechanism suitable
for p2p transactions.

To compile:

```sh
make compile-escrow
```

More info about the escrow in [cw-nride-escrow](cw-nride-escrow/README.md)

## IBC

Send 10 NRIDE tokens from `faucet` account on JUNO to `osmo-faucet` account on OSMOSIS via IBC:

```
make ics20-transfer from=$(junod keys show -a faucet) to=$(osmosisd keys show -a osmo-faucet) amount=10
```

Check balances:

```
make native-balance-osmo acc=osmo-faucet

balances:
- amount: "10"
  denom: ibc/E750D31033DC1CF4A044C3AA0A8117401316DC918FBEBC4E3D34F91B09D5F54C
- amount: "9997847"
  denom: uosmo
pagination:
  next_key: null
  total: "0"
```

Send tokens back:

```
make ics20-redeem from=$(osmosisd keys show -a osmo-faucet) to=$(junod keys show -a faucet) amount=5 
```

## Demo

We have two users, Alice and Bob, who will be using the escrow contract. Alice 
creates the escrow, locking 100 NRIDE tokens, with T1 timeout in 2 minutes, 
and T2 timeout in 5 minutes. Hence, Bob has 2 minutes to topup the contract, or 
else it will enter the T1-Timeout state where Alice can get her full deposit 
back. Once, Bob has topped up the escrow, both users have up to T2 timeout to 
approve or cancel the escrow. At any time, they can attempt to withdraw and see 
what the payout is. The contract returns an error if it is not in a withdrawable
state. Here, we will walk through the happy case, where both users approve the 
escrow on time, and get their full deposit back, but we encourage users to try 
cancelling or letting the escrow timeout to see how it affects the payout.

Note: This has only been tested on a Macbook Air with M1 processor

We currently support two environments "Local" and "Testnet", but we will run 
through the Local version, using a Docker container to run a single Juno node.

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

```
make demo-create
make demo-topup
make demo-approve-alice
make demo-approve-bob
make demo-withdraw
make demo-details
```

Note: run `make demo-details` at any time to query the escrow state

### Have fun!

Try using `make demo-cancel-alice` or `make demo-cancel-bob` to see what 
happens when one of the users cancels. Or try letting the escrow timeout to 
see what happens.


