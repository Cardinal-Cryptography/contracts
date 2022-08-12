## Address store example

This smart contract is part of an example showing how we can write upgradable smart contracts in ink! using `set_code_hash`.

### Description

This is a very simple smart contract, which provides methods that allow it's users to store some addresses in it's storage (using `add_new_address`),
and read the list of all addresses (using `get_addresses`).

### Upgradability

We can upgrade this contract by using `set_code` method, which allows us to assign new code to this contract's address, 
whilst preserving old storage (in this case it means that the new code will have access to our address list).

Check `efficient_address_store` and `named_address_store` examples for a more detailed description of how to upgrade this contract.
