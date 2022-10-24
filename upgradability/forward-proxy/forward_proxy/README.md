# `forward_proxy` contract
Contract implementing a simple forward-call proxy.

Contract instantiator is assigned admin role. This contract's administrator can transfer admin role to another account or change the logic contract.

...

Provided methods:
* `change_admin`
* `change_logic_contract`
* `_catch_all_forward`
