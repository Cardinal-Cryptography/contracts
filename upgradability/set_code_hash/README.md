# Smart contract upgrades with `set_code_hash`

## `set_code_hash` method

When using `set_code_hash` for upgrading contracts we basically tell `pallet-contracts` to assign the code of
a different contract to our current contract's address. This preserves old contract's storage.

## Example contracts

We provide examples which may help you familiarize yourself with the usage of this method, and some problems which
may arise in the process. Recommended order of reading the examples is the following:

### 1. `voting`

Smart contract which will be treated as a base for the upgrades.

### 2. `transparent_voting`

This example provides you with a contract which has the same exact storage layout as `voting`, but has additional functionality.
It shows how to perform a basic smart contract upgrade.

### 3. `voting_fixed`

Contract which has a different storage layout than `voting`. Shows how to avoid storage collisions and using uninitialized storage.
