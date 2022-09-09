# `address_book` example

This contract provides a possibility to store text data (bounded size) for a set of `AccountId`s.

One might use it as a place where account owners voluntarily share their contact info.

Messages:
- `set_info`: sets provided information as a stored data of the message caller.
- `get_info`: returns data stored for a specified `AccountId` (or `None` if it isn't present).
