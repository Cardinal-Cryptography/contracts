#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod address_book {
    use ink_storage::{traits::SpreadAllocate, Mapping};
    use ink_prelude::string::String;
    use scale::{Decode, Encode};

    #[derive(Eq, PartialEq, Debug, Decode, Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InfoTooLong,
    }

    const MAX_INFO_SIZE: usize = 20;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct AddressBook {
        contact_info: Mapping<AccountId, String>,
    }

    impl AddressBook {
        /// Initializes an empty contact info DB.
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|_| {})
        }

        /// Sets contact info of the caller.
        #[ink(message)]
        pub fn set_info(&mut self, info: String) -> Result<(), Error> {
            if info.len() > MAX_INFO_SIZE {
                return Err(Error::InfoTooLong);
            }

            self.contact_info.insert(self.env().caller(), &info);
            Ok(())
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
            let mut contact_db = AddressBook::new();
            set_caller::<DefaultEnvironment>(accounts.alice);

            assert_eq!(contact_db.set_info(String::from("Alice")), Ok(()));
            assert_eq!(contact_db.get_info(accounts.alice), Some(String::from("Alice")));
        }

        #[ink::test]
        fn len_bound_works() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut contact_db = AddressBook::new();
            set_caller::<DefaultEnvironment>(accounts.alice);

            assert_eq!(contact_db.set_info(String::from("Alice -------------------")), Err(Error::InfoTooLong));
            assert_eq!(contact_db.get_info(accounts.alice), None);
        }
    }
}
