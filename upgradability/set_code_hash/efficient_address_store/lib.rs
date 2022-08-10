#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod efficient_address_store {
    use ink_prelude::vec::Vec;
    use ink_env::set_code_hash;

    #[ink(storage)]
    pub struct EfficientAddressStore {
        addresses: Vec<AccountId>,
    }

    impl EfficientAddressStore {

        /// Initializes empty address store
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                addresses: Vec::new(),
            }
        }

        /// Performs simple storage migration from address_store contract
        /// Should be run right after the upgrade, preferably just once
        #[ink(message)]
        pub fn migrate(&mut self) {
            let temp_store = self.addresses.clone();
            self.addresses = Vec::new();

            for id in temp_store {
                if !self.addresses.contains(&id){
                    self.addresses.push(id);
                }
            }
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

            bytes[0] = 3;
            let non_entry = AccountId::from(bytes);

            address_store.add_new_address(entry_1);
            address_store.add_new_address(entry_2);
            address_store.add_new_address(entry_2);
            assert!(address_store.addresses.contains(&entry_2));
            assert!(!address_store.addresses.contains(&non_entry));
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

            let mut entry_1_cnt = 0;
            for entry in address_store.addresses {
                if entry == entry_1 {
                    entry_1_cnt += 1;
                }
            }

            assert_eq!(entry_1_cnt, 1);
        }
    }
}
