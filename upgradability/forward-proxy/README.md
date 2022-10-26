# Smart contract upgradability using forward proxy pattern

## Forward proxy
Forward proxy is a contract that has a catch-all method that forwards all non-matching calls to the earlier specified logic contract.

## Example contracts
* `forward_proxy`: contract implementing the proxy
* `prime_arithmetic_lib_v1` and `prime_arithmetic_lib_v2` example versions of stateless "library" contracts maintained with `forward_proxy`
