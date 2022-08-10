#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
mod named_address_store {
    use ink_prelude::vec::Vec;
    use ink_prelude::string::String;
    use ink_env::set_code_hash;

    pub const NEW_STORAGE_KEY: u32 = openbrush::storage_unique_key!(NewStorage);

    #[derive(Default, Debug)]
    #[openbrush::upgradeable_storage(NEW_STORAGE_KEY)]
    pub struct NewStorage {
        addresses: Vec<(AccountId, Option<String>)>,
        migration_performed: bool,
    }

    #[ink(storage)]
    pub struct NamedAddressStore {
        _old_storage: Vec<AccountId>,
        storage: NewStorage,
    }

    impl NamedAddressStore {
        /// Initializes empty address store
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                _old_storage: Vec::new(),
                storage: NewStorage{
                    addresses: Vec::new(),
                    migration_performed: false,
                }
            }
        }

        /// Performs data migration from the old format
        #[ink(message)]
        pub fn migrate(&mut self) {
            // We only want to perform a migration if it has not happened already
            if self.storage.migration_performed {
                return;
            }

            // Moving existing data to a new format
            let mut adr_vec = Vec::<AccountId>::new();
            for address in &self._old_storage {
                if !adr_vec.contains(&address) {
                    adr_vec.push(*address);
                    self.storage.addresses.push((*address, None));
                }
            }

            self.storage.migration_performed = true;
        }

        /// Sets name for a specified address
        #[ink(message)]
        pub fn set_name_for_address(&mut self, address: AccountId, name: String) {
            for entry in &mut self.storage.addresses {
                if entry.0 == address {
                    entry.1 = Some(name);
                    break;
                }
            }
        }

        /// Gets name for a specified address
        #[ink(message)]
        pub fn get_name_for_address(&self, address: AccountId) -> Option<String> {
            for entry in &self.storage.addresses {
                if entry.0 == address {
                    return entry.1.clone();
                }
            }
            None
        }

        /// Adds new address to store, but only if not already present
        #[ink(message)]
        pub fn add_new_address(&mut self, address: AccountId, name: Option<String>) {
            let (adr_vec, _): (Vec<AccountId>, Vec<_>) = self.storage.addresses.clone().into_iter().unzip();

            if !adr_vec.contains(&address) {
                self.storage.addresses.push((address, name));
            }
        }

        /// Returns stored addresses
        #[ink(message)]
        pub fn get_addresses(&self) -> Vec<AccountId> {
            let (adr_vec, _): (Vec<AccountId>, Vec<_>) = self.storage.addresses.clone().into_iter().unzip();
            adr_vec
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
        use ink_lang as ink;

        #[ink::test]
        fn adding_address_works() {
            let mut address_store = NamedAddressStore::new();
            let mut bytes: [u8; 32]  = [0; 32];

            bytes[0] = 1;
            let address_1 = AccountId::from(bytes);

            bytes[0] = 4;
            let address_2 = AccountId::from(bytes);

            address_store.add_new_address(address_1, None);
            address_store.add_new_address(address_2, None);
            address_store.add_new_address(address_2, Some(String::from("aleph")));
            assert!(address_store.storage.addresses.contains(&(address_2, None)));
            assert!(!address_store.storage.addresses.contains(&(address_2, Some(String::from("aleph")))));
        }

        #[ink::test]
        fn setting_name_works() {
            let mut address_store = NamedAddressStore::new();
            let mut bytes: [u8; 32]  = [0; 32];

            bytes[0] = 1;
            let address_1 = AccountId::from(bytes);

            bytes[0] = 4;
            let address_2 = AccountId::from(bytes);

            address_store.add_new_address(address_1, None);
            address_store.add_new_address(address_2, None);
            address_store.set_name_for_address(address_1, String::from("aleph"));
            assert_eq!(address_store.get_name_for_address(address_1), Some(String::from("aleph")));
        }
    }
}
