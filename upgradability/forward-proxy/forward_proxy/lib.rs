#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod forward_proxy {
    use scale::{Decode, Encode};
    use ink_env::call::Call;
    use ink_env as env;

    #[derive(Eq, PartialEq, Debug, Decode, Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        PermissionDenied,
        ContractCallError,
    }

    #[ink(storage)]
    pub struct ForwardProxy {
        logic_contract: AccountId,
        admin: AccountId,
    }

    impl ForwardProxy {
        #[ink(constructor)]
        pub fn new(logic_contract: AccountId, admin: AccountId) -> Self {
            Self { logic_contract, admin }
        }

        /// Allows admin to transfer his privilages to another account
        #[ink(message)]
        pub fn change_admin(&mut self, new_admin: AccountId) -> Result<(), Error> {
            if self.env().caller() != self.admin {
                Err(Error::PermissionDenied)
            } else {
                self.admin = new_admin;
                Ok(())
            }
        }

        /// Allows admin to change address of the logic contract
        #[ink(message)]
        pub fn change_logic_contract(&mut self, new_logic_contract: AccountId) -> Result<(), Error> {
            if self.env().caller() != self.admin {
                Err(Error::PermissionDenied)
            } else {
                self.logic_contract = new_logic_contract;
                Ok(())
            }
        }

        /// A catch-all method which forwards calls which selectors
        /// do not match other methods of that proxy
        #[ink(message, payable, selector = _)]
        pub fn _catch_all_forward(&self) -> Result<(), Error> {
            match env::call::build_call::<env::DefaultEnvironment>()
                .call_type(
                    Call::new()
                    .callee(self.logic_contract)
                    .transferred_value(self.env().transferred_value()),
                    )
                .call_flags(
                    env::CallFlags::default()
                    .set_forward_input(true)
                    .set_tail_call(false),
                    )
                .fire() {
                    Err(_) => Err(Error::ContractCallError),
                    _ => Ok(()),
                }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;
        use ink_env::test::{set_caller, default_accounts};
        use ink_env::DefaultEnvironment;

        #[ink::test]
        fn non_admin_cannot_change_admin() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut forward_proxy = ForwardProxy::new(accounts.frank, accounts.alice);

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(matches!(forward_proxy.change_admin(accounts.bob), Err(Error::PermissionDenied)));
        }

        #[ink::test]
        fn admin_can_change_admin() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut forward_proxy = ForwardProxy::new(accounts.frank, accounts.alice);

            set_caller::<DefaultEnvironment>(accounts.alice);
            assert!(forward_proxy.change_admin(accounts.bob).is_ok());
        }

        #[ink::test]
        fn non_admins_cannot_change_logic_contract() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut forward_proxy = ForwardProxy::new(accounts.frank, accounts.alice);

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(matches!(forward_proxy.change_logic_contract(accounts.eve), Err(Error::PermissionDenied)));
        }

        #[ink::test]
        fn admins_can_change_logic_contract() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut forward_proxy = ForwardProxy::new(accounts.frank, accounts.alice);

            set_caller::<DefaultEnvironment>(accounts.alice);
            assert!(forward_proxy.change_logic_contract(accounts.eve).is_ok());
        }
    }
}
