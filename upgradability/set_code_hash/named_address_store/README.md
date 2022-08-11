## Named address store example

This smart contract is part of an example showing how we can write upgradable smart contracts in ink! using `set_code_hash`.

See `address_store` and `efficient_address_store` examples first.

### Description

This contract provides a functionality of storing addresses (at most one copy of each), additionally users might assign 
addresses custom names, and query a name of a stored address (`set_name_for_address` and `get_name_for_address` respectively).

The goal of this example is to show how one might perform a smart contract upgrade (using `set_code_hash` method) in a case in which
storage layouts of the orginal contract and the new one differ.

### Upgrading `address_store` smart contract: upgrade with storage layout change

For an intorduction of how to upgrade with `set_code_hash` see `efficient_address_store` example.

#### Storage collisions

The big difference between the upgrade in `efficient_address_store` example, and in this one, is that here we want to change the storage layout.
In both `address_store` and `efficient_address_store` we just needed to keep a `Vec` of addresses, so both contract codes mapped this `Vec` to the 
same part of contract's storage. Now we want to store a `Vec` of pairs `(AccountId, Option<String>)`, so if we were to perform a simple upgrade
like before, we would not be able to read the old data.

This time we need to store the new data in a different place.

#### Uninitialized storage

Another problem which arises with a change of the storage layout is the fact, that right after the upgrade our new variables are uninitialized.
Since we just swap out the old code for the new one, the constructor of a new contract cannot be called, and we are not able to use our new storage.

#### `openbrush::upgradeable_storage` macro

Solving both of this problems might generate a quite a lot of boilerplate when writing in pure ink!, so in this example we use `upgradable_storage(STORAGE_KEY)` macro
from Openbrush.

It enables us to create a part of storage which is stored under the selected `STORAGE_KEY`, and additionally guards us against using uninitialized variables:
if such a variable were to be used, it is first initialized with a default value.

See the example code to familiarize yourself with the usage of this macro.

#### Migration

Unlike in `efficient_address_store` example, here migration/initialization of a new contract version is crucial.
Since old storage and new storage are disjoint, we need to perform a migration in order to be able to access the data through the new storage.

#### Instructions

See instructions for `efficient_address_store`.

### Upgradability

Similarly to `address_store` this smart contract provides `set_code` method, which enables us to change it's code, whilst preserving storage and address.
