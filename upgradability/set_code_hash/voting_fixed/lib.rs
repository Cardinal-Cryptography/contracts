#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
mod voting_fixed {
    use ink_storage::{traits::SpreadAllocate, Mapping};

    const STORAGE_KEY: u32 = openbrush::storage_unique_key!(NewVotes);
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
            ink_lang::utils::initialize_contract(|_| {})
        }

        /// Performs migration from `voting`
        /// Should be called once, right after the upgrade
        #[ink(message)]
        pub fn migrate(&mut self) {
            assert!(self.env().caller() == self._admin);
            self.votes.data[0] = self._old_votes[0] as u64;
            self.votes.data[1] = self._old_votes[1] as u64;
        }

        /// Current result of the vote
        #[ink(message)]
        pub fn get_winner(&self) -> u8 {
            return (self.votes.data[0] < self.votes.data[1]) as u8;
        }

        fn vote(&mut self, on: usize) {
            let voter = self.env().caller();
            assert!(self.voters.contains(voter));
            assert!(!self.voters.get(voter).unwrap());
            self.votes.data[on] += 1;
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
    }
}
