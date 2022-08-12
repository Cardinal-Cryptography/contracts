## Efficient address store example

This smart contract is part of an example showing how we can write upgradable smart contracts in ink! using `set_code_hash`.

See `address_store` example first.

### Description

This example provides a very similar functionality to the contract from `address_store` example. 
It allows it's users to store some addresses, and to read the list of all addresses stored.
However, unlike in `address_store`, it won't store an address if it is already present on the list.

The point of this example it to show how to perform a smart contract upgrade using `set_code_hash` method 
in a case in which storage layout of the new contract does not differ from the old one.

### Upgrading `address_store` smart contract: upgrade without changing storage layout

The difference between this contract, and `address_store` contract is in the fact, that this new one won't store
the same address multiple times. This way of storing addresses in more efficient, so we might 
want to upgrade the old contract to use the new logic, but we probably do not want to lose our data.

To upgrade to the new logic, it is enough to call method `set_code` with the hash of the new code. 
Because storage layout of this two contracts is identical, we do not need to make any additional 
adjustments to have this storage available in the new code.

#### Migrating

In some cases, old state won't match the logic of the new code, so we might want to perform some additional
steps after the upgrade.
In such a case we might want to provide a method `initialize` or `migrate`, which will need to be called right after
the upgrade.

With the code of our new `efficient_address_store` contract we would expect our storage to keep only one copy of each
address, but right after the upgrade this might not be the case: the old code did not follow this rule.
To enable us to fix the old storage, new code provides `migrate` method, which will erase unnecessary copies.

#### Instructions
- Build and deploy the `address_store` smart contract.
- Build and upload code of `efficient_address_store` smart contract.
- Call the `address_store` smart contract, pass hash of `efficient_address_store`'s code as an argument.
- Change the ABI of the deployed contract to the ABI of `efficient_address_store` (`Add an existing contract` with address of the contract and `metadata.json` of `efficient_address_store`).
- Call `migrate` through the new ABI.
- Now you can use contract in the upgraded version.

### Upgradability

Similarly to `address_store` this smart contract provides `set_code` method, which enables us to change it's code, whilst preserving storage and address. 
