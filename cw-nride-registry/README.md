# NRIDE Registry

This contract implements a simple registry for drivers. It is used in nRide's
discovery mechanism when riders look for drivers in their vicinity.

Each record in the registry contains:

- JUNO address
- NKN address
- Location

A driver can create or update their record (only the owner of the wallet 
corresponding to the JUNO address) to advertise their availability in that 
location.

Riders query the registry to get the list of all drivers in their vicinity 
(query by location) and broadcast a request to all these drivers, using the
NKN address field.

## Demo

Initialize the environment (cf readme in cw-nride-escrow for full instructions):

```
make demo-bootstrap
```

Subscribe alice and bob to 'paris' location, with dummy nkn addresses:

```
make registry-subscribe from=alice nkn=blablabla location=paris
make registry-subscribe from=bob nkn=chachacha location=paris
```

Query details:

```
make registry-details addr=$(junod keys show -a alice)
```

```
./scripts/registry-details.sh juno1agpgyrrvesdu62dmurgdnjt2uvjmflau6mlzqq
data:
  location: paris
  nkn_addr: blablabla
  reg_addr: juno1agpgyrrvesdu62dmurgdnjt2uvjmflau6mlzqq
```

Query by location:

```
make registry-list location=paris
```

```
./scripts/registry-list.sh paris
data:
- location: paris
  nkn_addr: blablabla
  reg_addr: juno1agpgyrrvesdu62dmurgdnjt2uvjmflau6mlzqq
- location: paris
  nkn_addr: chachacha
  reg_addr: juno1uddtnf3kz3wsa859fmjn6pss6y7y0h2rrj9puy
```