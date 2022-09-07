# Forward call example

These examples show one of the ways of using forward calls to achieve smart contract upgradability.

## Forward call

A cross-contract call which is executed in the context of an external contract, this execution won't read/write 
to the storage of the proxy contract (`address_book_aggregator` in this case).

## Example contracts

We provide two example contracts:

### `address_book`

A simple contract which role is to store text data for some `AccountIds` eg. contact info that owner of the account has voluntarily provided.

### `address_book_aggregator`

Contract that is able to store metadata of several `address_book` contracts and query them for info about specified `AccountId`.
Owner of the contract can add/modify contracts that the queries are forwarded to.
