# `address_book_aggregator` example

The goal of this example is to showcase possibility of using forward calls to achieve contract upgradability.

## Functionality

The role of this contract is to gather account information spread between several instances of `address_book` or another contracts providing similar
functionality.

It allows owner of the `address_book_aggregator` contract (instantiator) to add/change contracts which will be queried for info about accounts.

## Messages 
- `set_address_book`: Allows owner to provide `AccountId` of a contract that will be queried, along with the selector and the internal id (an array index).
- `get_info`: Performs forward calls to the contracts set by the owner, until it is able to retrieve information about a apecified account.

## Upgradability

Basic upgrade of this contract adds a new contract to the list of forwarded to contracts and thus enlarging the amount of account info that can be retrieved.

As the only reqiurement on the queried contracts is that they need to provide a message that takes `AccountId` and returns `Option<String>`, an owner of
the `address_book_aggregator` contract can change it's behaviour by changing set of the memorized contracts.
