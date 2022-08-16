## Transparent voting example

This smart contract is part of an example showing how we can write upgradable smart contracts in ink! using `set_code_hash`.

See `voting` example first.

### Description

This example provides a very similar functionality to the contract from `voting` example.
It provides a simple voting interface which has been extended with `votes_for_0` and
`votes_for_1`, which return current number of votes for the selected option.

The point of this example is to show how to perform a smart contract upgrade using `set_code_hash` method
in a case in which storage layout of the new contract does not differ from the old one.

### Upgrading `voting` smart contract: upgrade without changing storage layout

This contract provides additional functionality when compared to `voting` contract.
We might want to add the functionality of reading vote numbers, but we might have already
deployed `voting`, and gathered some votes, so we don't want to lose our voting data.

To upgrade to the new logic, it is enough to call method `set_code` with the hash of the new code.
Because storage layout of this two contracts is identical, we do not need to make any additional
adjustments to have this storage available in the new code.

#### Instructions
- Build and deploy the `voting` smart contract.
- Build and upload code of `transparent_voting` smart contract.
- Call the `voting` smart contract, pass hash of `transparent_voting`'s code as an argument.
- Change the ABI of the deployed contract to the ABI of `transparent_voting` (`Add an existing contract` with address of the contract and `metadata.json` of `transparent_voting`).
- Now you can use contract in the upgraded version.

### Upgradability

Similarly to `voting` this smart contract provides `set_code` method, which enables us to change it's code, whilst preserving storage and address.
