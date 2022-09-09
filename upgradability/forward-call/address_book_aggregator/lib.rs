#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod address_book_aggregator {
    use ink_env::call::{Call, ExecutionInput, Selector};
    use ink_prelude::string::String;
    use scale::{Decode, Encode};

    #[derive(Eq, PartialEq, Debug, Decode, Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        PermissionDenied,
        InvalidBookId,
    }

    type SelectorData = [u8; 4];
    type ExternalMessageData = Option<(AccountId, SelectorData)>;

    const MAX_BOOK_COUNT: usize = 5;
    const MAX_RETURNED_INFO_SIZE: usize = 20;

    #[ink(storage)]
    pub struct AddressBookAggregator {
        /// Owner of the contract, can change contracts we forward to.
        owner: AccountId,

        /// Ids of the contract we query, along with required selectors.
        address_books: [ExternalMessageData; MAX_BOOK_COUNT],
    }

    impl AddressBookAggregator {
        /// Inintializes the contract and sets it's owner.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                address_books: [None; MAX_BOOK_COUNT],
            }
        }

        /// Allows owner of this contract to add/modify one of
        /// the addresses that we forward to.
        #[ink(message)]
        pub fn set_address_book(&mut self, contract_id: AccountId, get_selector: SelectorData, book_id: u64) -> Result<(), Error> {
            if self.env().caller() != self.owner {
                return Err(Error::PermissionDenied);
            }

            if book_id as usize > MAX_BOOK_COUNT {
                return Err(Error::InvalidBookId);
            }

            self.address_books[book_id as usize] = Some((contract_id, get_selector));
            Ok(())
        }

        /// Allows owner of this contract to remove one of
        /// the addresses we forward to.
        #[ink(message)]
        pub fn remove_address_book(&mut self, book_id: u64) -> Result<(), Error> {
            if self.env().caller() != self.owner {
                return Err(Error::PermissionDenied);
            }

            if book_id as usize > MAX_BOOK_COUNT {
                return Err(Error::InvalidBookId);
            }

            self.address_books[book_id as usize] = None;
            Ok(())
        }

        /// A function which queries memorized contracts
        /// for contact info of a specified address.
        #[ink(message)]
        pub fn get_info(&self, account_id: AccountId) -> Option<String> {
            for id in (0..MAX_BOOK_COUNT).rev() {
                if let Some((forward_to, selector)) = self.address_books[id] {

                    // Here we perform a forward call to a contract that is supposed
                    // to store contact info for some addresses.
                    // If we are able to retrieve this info, then we retrurn it.
                    // Otherwise we will continue to search in the rest of "address books".
                    let res = ink_env::call::build_call::<ink_env::DefaultEnvironment>()
                        .call_type(
                            Call::new()
                            .callee(forward_to)
                            )
                        .exec_input(
                            ExecutionInput::new(Selector::new(selector))
                            .push_arg(account_id)
                            )
                        .returns::<Option<String>>()
                        .fire();

                    if let Ok(Some(info)) = res {
                        if info.len() <= MAX_RETURNED_INFO_SIZE {
                            return Some(info);
                        }
                    }
                }
            }
            None
        }
    }
}
