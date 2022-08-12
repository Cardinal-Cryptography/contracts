#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod voting {
    use ink_env::set_code_hash;
    use ink_storage::{traits::SpreadAllocate, Mapping};

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Voting {
        votes: [u8; 2],
        admin: AccountId,
        voters: Mapping<AccountId, bool>,
    }

    impl Voting {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.votes = [0,0];
                contract.admin = contract.env().caller();
            })
        }

        #[ink(message)]
        pub fn add_new_voter(&mut self, voter: AccountId) {
            assert!(self.env().caller() == self.admin);
            if !self.voters.contains(voter) {
                self.voters.insert_return_size(voter, &false);
            }
        }

        #[ink(message)]
        pub fn get_winner(&self) -> u8 {
            return (self.votes[0] < self.votes[1]) as u8;
        }

        fn vote(&mut self, on: usize) {
            let voter = self.env().caller();
            assert!(self.voters.contains(voter));
            assert!(!self.voters.get(voter).unwrap());
            self.votes[on] += 1;
            self.voters.insert(voter, &true);
        }

        /// Vote for option 0
        #[ink(message)]
        pub fn vote_0(&mut self) {
            self.vote(0);
        }

        /// Vote for option 1
        #[ink(message)]
        pub fn vote_1(&mut self) {
            self.vote(1);
        }

        /// Sets new code hash, updates contract code
        #[ink(message)]
        pub fn set_code(&mut self, code_hash: [u8; 32]) {
            assert!(self.env().caller() == self.admin);
            set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!(
                    "Failed to `set_code_hash` to {:?} due to {:?}",
                    code_hash, err
                )
            });
        }
    }
}
