# NRIDE Escrow

This is an escrow meta-contract that allows multiple users to create independent
escrows. Each escrow involves two users and has a unique id (for future calls to
reference it).

## How it works

Party A (the creator) locks some funds in an escrow, with a given recipient and
a lock. The lock is the public part of a cryptographic key-pair. Anyone with the
private key corresponding to the lock, can call the `withdraw` method to release
the funds to the recipient. At any time, the creator can call the `cancel` 
method to cancel the escrow and get their deposit back.

Note that only the creator can call the `cancel` method, but anyone can call the 
`withdraw` method as long as they have the secret key.

## Usage

Try the [demo](../README.md#demo) and study the [scripts](../scripts/escrow/).
