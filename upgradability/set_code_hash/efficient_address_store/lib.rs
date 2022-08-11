#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod efficient_address_store {
    use ink_prelude::vec::Vec;
    use ink_env::set_code_hash;

    #[derive(Default)]
    #[ink(storage)]
    pub struct EfficientAddressStore {
        addresses: Vec<AccountId>,
    }

    impl EfficientAddressStore {
        /// Initializes empty address store
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        /// Removes duplicate AccountIds from addresses
        /// Should be run after the upgrade from address_store
        #[ink(message)]
        pub fn migrate(&mut self) {
            self.addresses.sort();
            self.addresses.dedup();
        }

        /// Adds new address to store, but only if not already present
        #[ink(message)]
        pub fn add_new_address(&mut self, address: AccountId) {
            if !self.addresses.contains(&address) {
                self.addresses.push(address);
            }
        }

        /// Returns stored addresses
        #[ink(message)]
        pub fn get_addresses(&self) -> Vec<AccountId> {
            self.addresses.clone()
        }

        /// Sets new code hash, updates contract code
        #[ink(message)]
        pub fn set_code(&mut self, code_hash: [u8; 32]) {
            set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!(
                    "Failed to `set_code_hash` to {:?} due to {:?}",
                    code_hash, err
                )
            });
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_env::AccountId;
        use ink_lang as ink;

        #[ink::test]
        fn adding_address_works() {
            let mut address_store = EfficientAddressStore::new();
            let mut bytes: [u8; 32]  = [0; 32];

            bytes[0] = 1;
            let entry_1 = AccountId::from(bytes);
            bytes[0] = 4;
            let entry_2 = AccountId::from(bytes);

            address_store.add_new_address(entry_1);
            address_store.add_new_address(entry_2);
            address_store.add_new_address(entry_2);
            assert!(address_store.addresses.contains(&entry_2));
        }

        #[ink::test]
        fn efficiency_fix_works() {
            let mut address_store = EfficientAddressStore::new();
            let mut bytes: [u8; 32]  = [0; 32];

            bytes[0] = 1;
            let entry_1 = AccountId::from(bytes);
            bytes[0] = 4;
            let entry_2 = AccountId::from(bytes);

            address_store.add_new_address(entry_1);
            address_store.add_new_address(entry_2);
            address_store.add_new_address(entry_1);

            let entry_1_cnt = address_store.addresses.iter().filter(|&x| *x == entry_1).count();
            assert_eq!(entry_1_cnt, 1);
        }
    }
}
