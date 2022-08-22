#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod voting {
    use ink_env::set_code_hash;
    use ink_storage::{traits::SpreadAllocate, Mapping};
    use scale::{Decode, Encode};

    #[derive(Eq, PartialEq, Debug, Decode, Encode)]
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
            (self.votes[0] < self.votes[1]) as u8
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
            self.vote(0)
        }

        /// Vote for option 1
        #[ink(message)]
        pub fn vote_1(&mut self) -> Result {
            self.vote(1)
        }

        /// Sets new code hash, updates contract code
        #[ink(message)]
        pub fn set_code(&mut self, code_hash: [u8; 32]) -> Result {
            if self.env().caller() != self.admin {
                return Err(Error::PermissionDenied);
            }

            if set_code_hash(&code_hash).is_err() {
                return Err(Error::SetCodeFailed);
            };

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;
        use ink_env::test::{set_caller, default_accounts};
        use ink_env::DefaultEnvironment;

        #[ink::test]
        fn non_voters_cannot_vote() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut voting = Voting::new();

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(matches!(voting.vote_1(), Err(Error::NotVoter)));
        }

        #[ink::test]
        fn voters_can_vote() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut voting = Voting::new();
            assert!(voting.add_new_voter(accounts.bob).is_ok());

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(voting.vote_1().is_ok());
        }

        #[ink::test]
        fn non_admins_cannot_add_voters() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut voting = Voting::new();

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(matches!(voting.add_new_voter(accounts.charlie), Err(Error::PermissionDenied)));
        }

        #[ink::test]
        fn no_multiple_votes() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut voting = Voting::new();
            assert!(voting.add_new_voter(accounts.bob).is_ok());

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(voting.vote_1().is_ok());
            assert!(matches!(voting.vote_0(), Err(Error::HasAlreadyVoted)));
        }

        #[ink::test]
        fn simple_voting_scenario() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut voting = Voting::new();

            assert!(voting.add_new_voter(accounts.bob).is_ok());
            assert!(voting.add_new_voter(accounts.charlie).is_ok());
            assert!(voting.add_new_voter(accounts.django).is_ok());
            assert!(voting.add_new_voter(accounts.eve).is_ok());

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(voting.vote_0().is_ok());
            set_caller::<DefaultEnvironment>(accounts.charlie);
            assert!(voting.vote_1().is_ok());
            set_caller::<DefaultEnvironment>(accounts.django);
            assert!(voting.vote_0().is_ok());
            set_caller::<DefaultEnvironment>(accounts.eve);
            assert!(voting.vote_0().is_ok());

            assert!(voting.get_winner() == 0);
        }
    }
}
