#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod contact_proxy {
    use ink_env::call::{Call, ExecutionInput, Selector};
    use ink_prelude::string::String;

    #[ink(storage)]
    pub struct ContactProxy {
        /// Owner of the contract, can change forwarded to contract.
        owner: AccountId,

        /// AccountId of the forwarded to contract.
        db_address: AccountId,

        /// Selector of the method we forward to.
        db_get_selector: [u8; 4],
    }

    impl ContactProxy {
        /// Inintializes the proxy.
        /// Sets the owner of the contract and allows
        /// setting metadata of fotwarded to contract.
        #[ink(constructor)]
        pub fn new(db_address: AccountId, db_get_selector: [u8; 4]) -> Self {
            Self {
                owner: Self::env().caller(),
                db_address,
                db_get_selector,
            }
        }

        /// Allows owner of this contract to change the contract
        /// to which contact info queries are forwarded.
        #[ink(message)]
        pub fn change_db(&mut self, new_db_address: AccountId, db_get_selector: [u8; 4]) -> bool {
            if self.env().caller() != self.owner {
                return false;
            }

            self.db_address = new_db_address;
            self.db_get_selector = db_get_selector;
            true
        }

        /// A function which forwards to the selected method of the
        /// selected contract and queries it for contact info of specified address.
        /// Returns result of the query.
        #[ink(message)]
        pub fn get_info(&self, account_id: AccountId) -> Option<String> {
             ink_env::call::build_call::<ink_env::DefaultEnvironment>()
                .call_type(
                    Call::new()
                        .callee(self.db_address)
                        .transferred_value(0)
                        .gas_limit(0),
                )
                .exec_input(
                    ExecutionInput::new(Selector::new(self.db_get_selector))
                        .push_arg(account_id)
                    )
                .returns::<Option<String>>()
                .fire()
                .unwrap()
        }
    }
}

