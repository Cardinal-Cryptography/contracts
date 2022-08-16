## Voting example

This smart contract is part of an example showing how we can write upgradable smart contracts in ink! using `set_code_hash`.

### Description

Smart contract which provides a simple voting interface:
- `add_new_voter` allows contract's instantiator to add new accounts with voting rights.
- `vote_0` and `vote_1` allow users with voting rights to vote on selected option (each
accounts gets one vote).
- `get_winner` returns the current result of the vote.

### Upgradability

We can upgrade this contract by using `set_code` method, which allows contract's instantiator to assign new code to this contract's address,
whilst preserving old storage.

Check `transparent_voting` and `voting_fixed` examples for a more detailed description of how to upgrade this contract.
