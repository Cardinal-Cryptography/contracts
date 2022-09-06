#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod contact_db {
    use ink_storage::{traits::SpreadAllocate, Mapping};
    use ink_prelude::string::String;

    const MAX_INFO_LEN: usize = 20;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct ContactDb {
        contact_info: Mapping<AccountId, String>,
    }

    impl ContactDb {

        /// Initializes an empty contact info DB.
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|_| {})
        }

        /// Sets contact info of the caller.
        #[ink(message)]
        pub fn set_info(&mut self, info: String) -> bool {
            if info.len() > MAX_INFO_LEN {
                return false;
            }

            self.contact_info.insert(self.env().caller(), &info);
            true
        }

        /// Gets contact info of the specified address.
        #[ink(message)]
        pub fn get_info(&self, account_id: AccountId) -> Option<String> {
            self.contact_info.get(account_id)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;
        use ink_env::test::{set_caller, default_accounts};
        use ink_env::DefaultEnvironment;

        #[ink::test]
        fn simple_set_works() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut contact_db = ContactDb::new();
            set_caller::<DefaultEnvironment>(accounts.alice);

            assert!(contact_db.set_info(String::from("Alice")));
            assert_eq!(contact_db.get_info(accounts.alice), Some(String::from("Alice")));
        }

        #[ink::test]
        fn len_bound_works() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut contact_db = ContactDb::new();
            set_caller::<DefaultEnvironment>(accounts.alice);

            assert!(!contact_db.set_info(String::from("Alice -------------------")));
            assert_eq!(contact_db.get_info(accounts.alice), None);
        }
    }
}
