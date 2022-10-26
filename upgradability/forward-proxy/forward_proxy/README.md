# `forward_proxy` contract
Contract implementing a simple forward-call proxy.

## Forward proxy
Forward proxy is a contract which stores address of some other contract (actually deployed on chain, not just it's code hash) and provides a catch-all forwarding method.
It is a method which will redirect any calls that do not match proxy's messages' selectors to the stored address.

Such a proxy might, for example, be used to provide an interface to a stateless library that might need to be later upgraded.

Note: as all logic is performed in the context of the logic contract, the storage won't be persistent between upgrades.

## Instructions
* Deploy some other contract eg. `prime_arithmetic_lib_v1`.
* Deploy `forward_proxy` with `logic_contract` set to the address of previously deployed contract.
* Interacting through UI: update contract's metadata (simpliest way would be to "add existing contract" with address of the proxy and metadata of the logic contract).
* Use the logic contract through the proxy.
* Deploy updated version of the logic contract (eg. `prime_arithmetic_lib_v2`). 
* Call `change_logic_contract` providing address of the new contract (you may need to switch to proxy metadata again).
* Use the new contract.
* Note: if you do not intend to use the old contract, you might want to terminate it.

### Provided messages:
* `change_admin` - allows contract's admin to transfer privilages to another account. Throws `PermissionDenied` when called by non-admin.
* `change_logic_contract` - allows contract's admin to change address to which calls are to be forwarded. Throws `PermissionDenied` when called by non-admin. 
* `_catch_all_forward` - forwards all calls, which selectors' do not match those of `change_admin` or `change_logic_contract`, to earlier specified (logic contract's) address.
