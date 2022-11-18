# `address_book_aggregator` example

The goal of this example is to showcase usage of cross-contract calls.

## Functionality

The role of this contract is to gather account information spread between several instances of `address_book` or another contracts providing similar
functionality.

It allows owner of the `address_book_aggregator` contract (instantiator) to add/change contracts which will be queried for info about accounts.

## Messages
- `set_address_book`: Allows owner to provide `AccountId` of a contract that will be queried, along with the selector and the internal id (an array index).
- `get_info`: Performs forward calls to the contracts set by the owner, until it is able to retrieve information about a specified account.

