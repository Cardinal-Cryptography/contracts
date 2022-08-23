# Voting fixed example

This smart contract is part of an example showing how we can write upgradable smart contracts in ink! using `set_code_hash`.

See `voting` and `transparent_voting` examples first.

## Description

This contract provides a simple voting interface which allows voting for one of some two options and checking current number of votes.
However, when compared to `transparent_voting`, this contract has ability to store more votes and forbids adding new voters or
upgrading itself.

The goal of this example is to show how one might perform a smart contract upgrade (using `set_code_hash` method) in a case in which
storage layouts of the original contract and the new one differ.

## Upgrading `transparent_voting` smart contract: upgrade with storage layout change

For an introduction of how to upgrade with `set_code_hash` see `transparent_voting` example.

### Example scenario

Let's say you deployed `transparent_voting` contract (or you have upgraded `voting` to `transparent_voting`),
updated list of the allowed voters to it's
desired state and gathered some votes. Then you realized that the contract has a flaw:
votes are stored in `u8` counters, and will soon overflow.

Now, you want to fix this issue changing the counters to `u64`, and remove ability
to add new voters/upgrade the contract (to assure users that the list of voters is final).

### Storage collisions

The big difference between the upgrade in `transparent_voting` example, and in this one, is that here we want to change the storage layout.
We cannot simply change counters' type to `u64`, we would not be able to read any of our
old data, this time we need to store the new data in a different place. One way of doing it is to
append new fields at the end of our storage struct, another is to keep the new data under
a different storage key.

### Uninitialized storage

Another problem which arises with adding new storage fields is the fact, that right after the upgrade our new variables are uninitialized.
Since we just swap out the old code for the new one, the constructor of a new contract cannot be called, and we are not able to use our new storage.

### `openbrush::upgradeable_storage` macro

Solving both of these problems might generate a quite a lot of boilerplate when writing in pure ink!, so in this example we use `upgradeable_storage(STORAGE_KEY)` macro (https://github.com/Supercolony-net/openbrush-contracts/blob/main/lang/macro/src/lib.rs#L447)
from Openbrush.

It enables us to create a part of storage which is stored under the selected `STORAGE_KEY`, and additionally guards us against using uninitialized variables:
if such a variable were to be used, it is first initialized with a default value.

See the example code to familiarize yourself with the usage of this macro.

### Migration

Unlike in `transparent_voting` example, here migration/initialization of a new contract version is crucial.
Since old storage and new storage are disjoint, we need to perform a migration in order to be able to access the data through the new storage.
Here, it is simply moving votes from old counters to the new ones, which is done by the
`migrate` method.

### Instructions
- Build and deploy the `transparent_voting` smart contract.
- Build and upload code of `voting_fixed` smart contract.
- Use the `transparent_voting` contract.
- Call the `transparent_voting` smart contract, pass hash of `voting_fixed` code as an argument.
- Change the ABI of the deployed contract to the ABI of `voting_fixed` (`Add an existing contract` with address of the contract and `metadata.json` of `voting_fixed`).
- Call `migrate` on the contract.
- Now you can use contract in the upgraded version.

Note: Since `transparent_voting` and `voting` have the same storage layout, you can use
this contract to upgrade both of the previous examples.

## No longer upgradeable

Since this contract does not provide an interface to call `set_code_hash`, it is no longer upgradeable.
