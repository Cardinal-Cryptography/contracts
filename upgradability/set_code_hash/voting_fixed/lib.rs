#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
mod voting_fixed {
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

    const STORAGE_KEY: u32 = openbrush::storage_unique_key!(NewVotes);
    #[derive(Debug)]
    #[openbrush::upgradeable_storage(STORAGE_KEY)]
    struct NewVotes{
        data: [u64; 2],
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct VotingFixed {
        _old_votes: [u8; 2],
        _admin: AccountId,
        voters: Mapping<AccountId, bool>,
        votes: NewVotes,
    }

    impl VotingFixed {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract._old_votes = [0,0];
                contract._admin = contract.env().caller();
                contract.votes.data = [0,0];
            })
        }

        /// Performs migration from `voting`
        /// Should be called once, right after the upgrade
        #[ink(message)]
        pub fn migrate(&mut self) -> Result {
            if self.env().caller() != self._admin {
                return Err(Error::PermissionDenied);
            }

            self.votes.data[0] = self._old_votes[0] as u64;
            self.votes.data[1] = self._old_votes[1] as u64;
            Ok(())
        }

        /// Current result of the vote
        #[ink(message)]
        pub fn get_winner(&self) -> u8 {
            (self.votes.data[0] < self.votes.data[1]) as u8
        }

        fn vote(&mut self, on: usize) -> Result {
            let voter = self.env().caller();
            if !self.voters.contains(voter) {
                return Err(Error::NotVoter);
            }

            if self.voters.get(voter).unwrap() {
                return Err(Error::HasAlreadyVoted);
            }

            self.votes.data[on] += 1;
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

        /// Get number of votes for option 0
        #[ink(message)]
        pub fn votes_for_0(&self) -> u64 {
            self.votes.data[0]
        }

        /// Get number of votes for option 1
        #[ink(message)]
        pub fn votes_for_1(&self) -> u64 {
            self.votes.data[1]
        }

        // Following methods are not messages and can be used for tests only

        /// Method for adding new voter.
        pub fn _old_add_new_voter(&mut self, voter: AccountId) -> Result {
            if self.voters.contains(voter) {
                return Err(Error::AlreadyVoter);
            }

            self.voters.insert(voter, &false);
            Ok(())
        }

        fn _old_vote(&mut self, on: usize) -> Result {
            let voter = self.env().caller();
            if !self.voters.contains(voter) {
                return Err(Error::NotVoter);
            }

            if self.voters.get(voter).unwrap() {
                return Err(Error::HasAlreadyVoted);
            }

            self._old_votes[on] += 1;
            self.voters.insert(voter, &true);
            Ok(())
        }

        /// Vote for option 0 in old storage
        pub fn _old_vote_0(&mut self) -> Result {
            Ok(self._old_vote(0)?)
        }

        /// Vote for option 1 in old storage
        pub fn _old_vote_1(&mut self) -> Result {
            Ok(self._old_vote(1)?)
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
            let mut voting = VotingFixed::new();

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(matches!(voting.vote_1(), Err(Error::NotVoter)));
        }

        #[ink::test]
        fn voters_can_vote() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut voting = VotingFixed::new();
            assert!(voting._old_add_new_voter(accounts.bob).is_ok());

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(voting.vote_1().is_ok());
        }

        #[ink::test]
        fn no_multiple_votes() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut voting = VotingFixed::new();
            assert!(voting._old_add_new_voter(accounts.bob).is_ok());

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(voting.vote_1().is_ok());
            assert!(matches!(voting.vote_0(), Err(Error::HasAlreadyVoted)));
        }

        #[ink::test]
        fn voting_scenario_with_upgrade() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut voting = VotingFixed::new();

            assert!(voting._old_add_new_voter(accounts.alice).is_ok());
            assert!(voting._old_add_new_voter(accounts.bob).is_ok());
            assert!(voting._old_add_new_voter(accounts.charlie).is_ok());
            assert!(voting._old_add_new_voter(accounts.django).is_ok());
            assert!(voting._old_add_new_voter(accounts.eve).is_ok());

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(voting._old_vote_0().is_ok());
            set_caller::<DefaultEnvironment>(accounts.charlie);
            assert!(voting._old_vote_1().is_ok());
            set_caller::<DefaultEnvironment>(accounts.django);
            assert!(voting._old_vote_0().is_ok());
            set_caller::<DefaultEnvironment>(accounts.eve);
            assert!(voting._old_vote_0().is_ok());

            // Simulated upgrade
            assert!(matches!(voting.migrate(), Err(Error::PermissionDenied)));
            set_caller::<DefaultEnvironment>(accounts.alice);
            assert!(voting.migrate().is_ok());
            assert!(voting.vote_1().is_ok());

            assert!(voting.get_winner() == 0);
            assert!(voting.votes_for_0() == 3);
            assert!(voting.votes_for_1() == 2);
        }
    }
}
