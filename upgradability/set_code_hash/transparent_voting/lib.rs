#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod transparent_voting {
    use ink_env::set_code_hash;
    use ink_storage::{traits::SpreadAllocate, Mapping};
    use scale::{Decode, Encode};

    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        PermissionDenied,
        NotVoter,
        HasAlreadyVoted,
        AlreadyVoter,
        SetCodeFailed,
    }

    pub type Result = core::result::Result<(), Error>;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct TransparentVoting {
        votes: [u8; 2],
        admin: AccountId,
        voters: Mapping<AccountId, bool>,
    }

    impl TransparentVoting {

        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.votes = [0,0];
                contract.admin = contract.env().caller();
            })
        }

        /// Adds a new voter, can be performed only by contract instantiator and only once for each account
        #[ink(message)]
        pub fn add_new_voter(&mut self, voter: AccountId) -> Result {
            if self.env().caller() != self.admin {
                return Err(Error::PermissionDenied);
            }

            if self.voters.contains(voter) {
                return Err(Error::AlreadyVoter);
            }

            self.voters.insert(voter, &false);
            Ok(())
        }

        /// Current result of the vote
        #[ink(message)]
        pub fn get_winner(&self) -> u8 {
            return (self.votes[0] < self.votes[1]) as u8;
        }

        fn vote(&mut self, on: usize) -> Result {
            let voter = self.env().caller();
            if !self.voters.contains(voter) {
                return Err(Error::NotVoter);
            }

            if self.voters.get(voter).unwrap() {
                return Err(Error::HasAlreadyVoted);
            }

            self.votes[on] += 1;
            self.voters.insert(voter, &true);
            Ok(())
        }

        /// Vote for option 0
        #[ink(message)]
        pub fn vote_0(&mut self) -> Result {
            self.vote(0)?;
            Ok(())
        }

        /// Vote for option 1
        #[ink(message)]
        pub fn vote_1(&mut self) -> Result {
            self.vote(1)?;
            Ok(())
        }

        /// Get number of votes for option 0
        #[ink(message)]
        pub fn votes_for_0(&self) -> u8 {
            self.votes[0]
        }

        /// Get number of votes for option 1
        #[ink(message)]
        pub fn votes_for_1(&self) -> u8 {
            self.votes[1]
        }

        /// Sets new code hash, updates contract code
        #[ink(message)]
        pub fn set_code(&mut self, code_hash: [u8; 32]) -> Result {
            if self.env().caller() != self.admin {
                return Err(Error::PermissionDenied);
            }

            if let Err(_) = set_code_hash(&code_hash) {
                return Err(Error::SetCodeFailed);
            };

            Ok(())
        }

    }
}
