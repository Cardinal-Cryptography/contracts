# Smart contract upgrades with `set_code_hash`

### `set_code_hash` method

When using `set_code_hash` for upgrading contracts we basically tell `pallet-contracts` to assign the code of
a different contract to our current contract's address. This preserves old contract's storage.

### Example contracts

We provide examples which may help you familiarize yourself with the usage of this method, and some problems which
may arise in the process. Recommendend order of reading the examples is the following:

#### 1. `address_store`

Smart contract which will be treated as a base for the upgrades.

#### 2. `efficient_address_store`

This example provides you with a contract which has the same exact storage layout as `address_store`, but different logic.
It shows how to perform a basic smart contract upgrade.

#### 3. `named_address_store`

Contract which has a different storage layout than `address_store`. Shows how to avoid storage collisions and using uninitialized storage.
