# IBC

This repo contains some scripts to transfer tokens to and from Osmosis. Here's
and example on how to use them.

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

