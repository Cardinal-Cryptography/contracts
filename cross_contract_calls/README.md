# Cross-contract calls examples

These examples showcase usage of cross-contract calls.

## Example contracts

We provide two example contracts:

### `address_book`

A simple contract which role is to store text data for some `AccountIds` eg. contact info that owner of the account has voluntarily provided.

### `address_book_aggregator`

Contract that is able to store metadata of several `address_book`s (or different contracts providing similar functionality)
and query them for info about specified `AccountId`.
Owner of the contract can add/remove/change which contracts queries will be forwarded to.
