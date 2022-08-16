//! # Escrow
//!
//! This implements an Escrow contract.
//!
//! ## Warning
//!
//! This contract is an *example*. It is neither audited nor endorsed for production use.
//! Do **not** rely on it to keep anything of value secure.
//!
//! ## Overview
//!
//! Escrow is the third party which holds the asset(asset can be money, bond, stocks)
//! on the presence of two parties. Escrow will release the fund when certain conditions are met.
//!
//! In this contract, there are three parties, the escrow, a buyer, and a seller. The buyer wants
//! to but some goods from the seller, and use this smart contract instance as trusted entity for
//! deposit funds.
//!
//! There are two outcomes of this SC, either delivery of the goods is marked as done, or funds are
//! are returned. First action can be made only as a seller, and the second one as either seller, or
//! via the buyer but only if some predefined time passed. This makes possible to unlock funds if the
//! seller delays confirmation of the delivery.
//! The deposit action can be done only by the buyer.

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod escrow {
    // use ink_prelude::string::{String, ToString};
    use ink_storage::traits::{PackedLayout, SpreadLayout};
    use scale::{Decode, Encode};

    /// The escrow's state, which might be one of the following
    /// * `AwaitPayment` - the buyer needs to deposit their funds first. It is also the initial
    /// state of an escrow contract, from which moment the clock begins.
    /// * `AwaitDelivery` - after funds are deposited, seller needs either to confirm delivery
    /// or to refund the deposit,
    /// * `Completed` - at this stage the funds are either transferred to the seller, or returned
    /// to the buyer. The escrow contract can end up in this state also when given number of blocks
    /// passed from the contract instantiation.
    // TODO do we need all attribs?
    #[derive(Debug, Encode, Decode, Clone, Copy, SpreadLayout, PackedLayout, PartialEq, Eq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub enum State {
        AwaitPayment,
        AwaitDelivery,
        Completed,
    }

    #[ink(storage)]
    pub struct Escrow {
        /// The escrow's state
        state: State,

        /// Buyer's account
        buyer: AccountId,

        /// Sellers's account
        seller: AccountId,

        /// Number of a block when contract is deemed as terminated. From this moment if seller
        /// did not claim a delivery, deposit can be requested back by the buyer
        escrow_end_time_at_block: u64,
    }

    /// An event emitted when the buyer has deposited some funds
    #[ink(event)]
    pub struct Deposit {
        #[ink(topic)]
        buyer: AccountId,
    }

    /// An event emitted when the seller has marked delivery as done
    #[ink(event)]
    pub struct Delivery {
        #[ink(topic)]
        seller: AccountId,
    }

    /// An event emitter in two situations after the buyer deposited the funds: either the seller
    /// claimed a delivery as done or predefined number of blocks passed from the escrow contract
    /// instantiation and seller has not marked delivery as done yet, upon the which deposit is
    /// returned to the buyer
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    /// Describes possible spectrum of error scenarios
    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Someone else than buyer tries to deposit funds
        DepositFundsNotAsBuyer,

        /// Someone else than seller tries to claim delivery as done
        ConfirmDeliveryNotAsSeller,

        /// The buyer already deposited the funds
        FundsAlreadyDeposited,

        /// When the seller tries to confirm a delivery and the buyer has not deposited funds yet
        FundsNotDepositedYet,

        /// All possible scenarios in which any action is done on the escrow contract which is
        /// deemed as completed
        ContractAlreadyCompleted,

        /// Either when not buyer tries to claim back funds or the buyer tries to do it before
        /// contract completed
        FundsCannotBeReturned,

        /// requested transfer failed, this can be the case if the contract does not
        /// have sufficient free funds or if the transfer would have brought the
        /// contract's balance below minimum balance
        TransferFailed,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Escrow {
        /// Instantiates new escrow contract with given buyer and seller accounts
        #[ink(constructor)]
        pub fn new(buyer: AccountId, seller: AccountId, escrow_period_in_blocks: u64) -> Self {
            let current_block: u64 = Self::env().block_number().into();
            Self {
                state: State::AwaitPayment,
                buyer,
                seller,
                escrow_end_time_at_block: current_block + escrow_period_in_blocks,
            }
        }

        fn mark_contract_as_completed_when_time_passed(&mut self) {
            let current_block: u64 = Self::env().block_number().into();
            if current_block > self.escrow_end_time_at_block {
                self.state = State::Completed;
            }
        }

        fn check_if_contract_not_completed_yet(&self) -> Result<()> {
            if self.state == State::Completed {
                return Err(Error::ContractAlreadyCompleted);
            }
            return Ok(());
        }

        fn make_transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            self.env()
                .transfer(to, value)
                .map_err(|_| Error::TransferFailed)?;
            self.env().emit_event(Transfer { to, value });
            Ok(())
        }

        /// Returns contract termination date, is block after which the contract is deemed as completed
        /// even when buyer or seller have not made their actions
        #[ink(message)]
        pub fn get_contract_termination_time(&self) -> u64 {
            self.escrow_end_time_at_block
        }

        /// Deposit funds for sake of buying some goods
        /// This can be done only by the buyer
        #[ink(message, payable)]
        pub fn deposit(&mut self) -> Result<()> {
            self.mark_contract_as_completed_when_time_passed();
            self.check_if_contract_not_completed_yet()?;
            let caller = Self::env().caller();
            if caller != self.buyer {
                return Err(Error::DepositFundsNotAsBuyer);
            }
            if self.state != State::AwaitPayment {
                return Err(Error::FundsAlreadyDeposited);
            }
            self.state = State::AwaitDelivery;

            Ok(())
        }

        /// Confirm that a delivery has made through, upon which funds are transferred to the seller
        /// This can be done only by the seller
        #[ink(message)]
        pub fn confirm_delivery(&mut self) -> Result<()> {
            self.mark_contract_as_completed_when_time_passed();
            self.check_if_contract_not_completed_yet()?;
            let caller = Self::env().caller();
            if caller != self.seller {
                return Err(Error::ConfirmDeliveryNotAsSeller);
            }
            if self.state != State::AwaitDelivery {
                return Err(Error::FundsNotDepositedYet);
            }
            let value = self.env().balance();
            self.make_transfer(self.seller, value)?;
            self.state = State::Completed;

            Ok(())
        }

        /// Refunds back a deposit when certain conditions are met
        /// * caller is the buyer,
        /// * the seller has not marked a delivery as done,
        /// * contract has terminated.
        #[ink(message)]
        pub fn refund_deposit(&mut self) -> Result<()> {
            self.mark_contract_as_completed_when_time_passed();
            if self.check_if_contract_not_completed_yet().is_ok() {
                return Err(Error::FundsCannotBeReturned);
            }

            let caller = Self::env().caller();
            if caller != self.buyer {
                return Err(Error::FundsCannotBeReturned);
            }

            let value = self.env().balance();
            if value == 0 {
                return Err(Error::FundsCannotBeReturned);
            }

            self.make_transfer(self.buyer, value)?;

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        // due to https://github.com/paritytech/ink/issues/1117, it's not possible to test
        // off-chain account balances via payable attribute
        // therefore in various places we simulate `payable` via setting balance on contract
        // constructor account

        fn get_default_test_accounts() -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment>
        {
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
        }

        fn advance_n_blocks(n: u64) {
            for _ in 0..n {
                ink_env::test::advance_block::<ink_env::DefaultEnvironment>();
            }
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(account_id, balance)
        }

        fn get_balance(account_id: AccountId) -> Balance {
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(account_id)
                .expect("Cannot get account balance")
        }

        fn contract_id() -> AccountId {
            ink_env::test::callee::<ink_env::DefaultEnvironment>()
        }

        fn set_caller(caller: AccountId) {
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(caller);
        }

        fn create_contract(
            initial_balance: Balance,
            period_in_blocks: u64,
        ) -> (Escrow, AccountId, AccountId, AccountId) {
            let accounts = get_default_test_accounts();
            let contract_founder = accounts.alice;
            let buyer = accounts.eve;
            let seller = accounts.frank;

            set_caller(contract_founder);
            set_balance(contract_id(), initial_balance);
            let escrow = Escrow::new(buyer, seller, period_in_blocks);
            assert_eq!(escrow.buyer, buyer);
            assert_eq!(escrow.seller, seller);
            assert_eq!(get_balance(contract_id()), initial_balance);
            assert_eq!(escrow.state, State::AwaitPayment);
            (escrow, contract_founder, buyer, seller)
        }

        fn deposit(
            escrow: &mut Escrow,
            buyer: AccountId,
            initial_buyer_balance: Balance,
            deposit: Balance,
        ) -> Result<()> {
            assert_eq!(escrow.state, State::AwaitPayment);

            // workaround for payable not working as expected in off-chain testing
            // see comment at the top of `mod tests`
            set_balance(buyer, initial_buyer_balance);

            ink_env::test::set_value_transferred::<ink_env::DefaultEnvironment>(deposit);
            set_caller(buyer);
            let result = escrow.deposit();
            if result.is_ok() {
                assert_eq!(escrow.state, State::AwaitDelivery);
            }
            result
        }

        fn confirm_delivery(
            escrow: &mut Escrow,
            seller: AccountId,
            expected_deposit_value: Balance,
        ) -> Result<()> {
            assert_eq!(escrow.state, State::AwaitDelivery);
            assert_eq!(get_balance(contract_id()), expected_deposit_value);
            set_caller(seller);
            let result = escrow.confirm_delivery();
            if result.is_ok() {
                assert_eq!(escrow.state, State::Completed);
                assert_eq!(get_balance(seller), expected_deposit_value);
                assert_eq!(get_balance(contract_id()), 0);
            }
            result
        }

        fn refund_deposit(
            escrow: &mut Escrow,
            buyer: AccountId,
            expected_deposit_value: Balance,
            expected_buyers_balance: Balance,
        ) -> Result<()> {
            assert_eq!(escrow.state, State::Completed);
            assert_eq!(get_balance(contract_id()), expected_deposit_value);
            set_caller(buyer);
            let result = escrow.refund_deposit();
            if result.is_ok() {
                assert_eq!(escrow.state, State::Completed);
                assert_eq!(get_balance(buyer), expected_buyers_balance);
                assert_eq!(get_balance(contract_id()), 0);
            }
            result
        }

        #[ink::test]
        fn given_new_escrow_contract_when_constructor_is_called_then_contract_is_initialized() {
            let (escrow, _, _, _) = create_contract(0, 100);
            assert_eq!(escrow.get_contract_termination_time(), 100);
        }

        #[ink::test]
        fn given_new_escrow_contract_when_block_advances_long_enough_then_contract_completes() {
            let (mut escrow, _, _, _) = create_contract(0, 100);
            assert_eq!(escrow.check_if_contract_not_completed_yet(), Ok(()));
            advance_n_blocks(1);
            escrow.mark_contract_as_completed_when_time_passed();
            assert_eq!(escrow.check_if_contract_not_completed_yet(), Ok(()));
            advance_n_blocks(99);
            escrow.mark_contract_as_completed_when_time_passed();
            assert_eq!(escrow.check_if_contract_not_completed_yet(), Ok(()));
            advance_n_blocks(1);
            escrow.mark_contract_as_completed_when_time_passed();
            assert_eq!(
                escrow.check_if_contract_not_completed_yet(),
                Err(Error::ContractAlreadyCompleted)
            );
            advance_n_blocks(20000);
            escrow.mark_contract_as_completed_when_time_passed();
            assert_eq!(
                escrow.check_if_contract_not_completed_yet(),
                Err(Error::ContractAlreadyCompleted)
            );
        }

        #[ink::test]
        fn given_new_escrow_contract_when_deposit_is_made_then_contract_balance_is_equal_to_deposit(
        ) {
            let (mut escrow, constructor_account, buyer, _) = create_contract(0, 100);
            assert_eq!(constructor_account, contract_id());
            assert_eq!(get_balance(constructor_account), 0);
            assert_eq!(deposit(&mut escrow, buyer, 100, 10), Ok(()));

            // ideally, rest of this test would do below, but returns 0 instead
            // see comment at the top of `mod tests`
            // assert_eq!(get_balance(constructor_account), 10);
        }

        #[ink::test]
        fn given_new_escrow_contract_when_deposit_is_made_more_than_once_then_error_is_returned() {
            let (mut escrow, _, buyer, _) = create_contract(10, 100);
            set_balance(buyer, 199);
            set_caller(buyer);
            ink_env::test::set_value_transferred::<ink_env::DefaultEnvironment>(100);
            assert_eq!(deposit(&mut escrow, buyer, 199, 10), Ok(()));
            assert_eq!(escrow.deposit(), Err(Error::FundsAlreadyDeposited));
        }

        #[ink::test]
        fn given_new_escrow_contract_when_deposit_is_made_not_by_buyer_then_error_is_returned() {
            let (mut escrow, _, _, seller) = create_contract(10, 100);
            let accounts = get_default_test_accounts();
            let impersonated_buyer = accounts.alice;
            assert_eq!(
                deposit(&mut escrow, impersonated_buyer, 98127345, 101),
                Err(Error::DepositFundsNotAsBuyer)
            );

            let impersonated_buyer = seller;
            assert_eq!(
                deposit(&mut escrow, impersonated_buyer, 19810238, 102221),
                Err(Error::DepositFundsNotAsBuyer)
            );
        }

        #[ink::test]
        fn given_completed_escrow_contract_when_deposit_is_made_by_buyer_then_error_is_returned() {
            let (mut escrow, _, buyer, _) = create_contract(10, 1000);
            advance_n_blocks(1001);
            set_balance(buyer, 199);
            ink_env::test::set_value_transferred::<ink_env::DefaultEnvironment>(10);
            set_caller(buyer);
            assert_eq!(escrow.deposit(), Err(Error::ContractAlreadyCompleted));
        }

        #[ink::test]
        fn given_new_escrow_contract_when_deposit_is_made_and_delivery_done_then_seller_receives_deposit(
        ) {
            let (mut escrow, _, buyer, seller) = create_contract(10, 1000);
            assert_eq!(deposit(&mut escrow, buyer, 199, 10), Ok(()));
            assert_eq!(confirm_delivery(&mut escrow, seller, 10), Ok(()));
        }

        #[ink::test]
        fn given_new_escrow_contract_when_deposit_is_not_made_and_delivery_done_then_error_is_returned(
        ) {
            let (mut escrow, _, _, seller) = create_contract(10, 1000);
            set_caller(seller);
            assert_eq!(escrow.confirm_delivery(), Err(Error::FundsNotDepositedYet));
        }

        #[ink::test]
        fn given_new_escrow_contract_when_delivery_done_as_not_seller_then_error_is_returned() {
            let (mut escrow, _, buyer, _) = create_contract(10, 1000);
            set_caller(buyer);
            assert_eq!(
                escrow.confirm_delivery(),
                Err(Error::ConfirmDeliveryNotAsSeller)
            );
        }

        #[ink::test]
        fn given_completed_escrow_contract_with_deposit_when_delivery_done_then_error_is_returned()
        {
            let (mut escrow, _, buyer, seller) = create_contract(10, 1000);
            assert_eq!(deposit(&mut escrow, buyer, 199, 10), Ok(()));
            advance_n_blocks(1001);
            set_balance(seller, 199);
            ink_env::test::set_value_transferred::<ink_env::DefaultEnvironment>(10);
            set_caller(seller);
            assert_eq!(
                escrow.confirm_delivery(),
                Err(Error::ContractAlreadyCompleted)
            );
        }

        #[ink::test]
        fn given_completed_escrow_contract_without_deposit_when_delivery_done_then_error_is_returned(
        ) {
            let (mut escrow, _, _, seller) = create_contract(10, 1000);
            advance_n_blocks(1001);
            set_balance(seller, 199);
            ink_env::test::set_value_transferred::<ink_env::DefaultEnvironment>(10);
            set_caller(seller);
            assert_eq!(
                escrow.confirm_delivery(),
                Err(Error::ContractAlreadyCompleted)
            );
        }

        #[ink::test]
        fn given_deposited_escrow_contract_completes_via_time_when_refund_deposit_is_called_then_deposit_is_returned(
        ) {
            let (mut escrow, _, buyer, _) = create_contract(10, 99);
            assert_eq!(deposit(&mut escrow, buyer, 199, 10), Ok(()));

            advance_n_blocks(100);
            escrow.mark_contract_as_completed_when_time_passed();

            assert_eq!(refund_deposit(&mut escrow, buyer, 10, 209), Ok(()));
        }

        #[ink::test]
        fn given_not_deposited_escrow_contract_completes_via_time_when_refund_deposit_is_called_then_deposit_is_not_returned(
        ) {
            let (mut escrow, _, buyer, _) = create_contract(0, 99);

            advance_n_blocks(100);
            escrow.mark_contract_as_completed_when_time_passed();

            assert_eq!(
                refund_deposit(&mut escrow, buyer, 0, 209),
                Err(Error::FundsCannotBeReturned)
            );
        }

        #[ink::test]
        fn given_deposited_escrow_contract_not_completes_via_time_when_refund_deposit_is_called_then_deposit_is_not_returned(
        ) {
            let (mut escrow, _, buyer, _) = create_contract(10, 99);
            assert_eq!(deposit(&mut escrow, buyer, 199, 10), Ok(()));

            advance_n_blocks(50);
            escrow.mark_contract_as_completed_when_time_passed();

            set_caller(buyer);
            assert_eq!(escrow.refund_deposit(), Err(Error::FundsCannotBeReturned));
        }

        #[ink::test]
        fn given_deposited_escrow_contract_not_completes_via_time_when_refund_deposit_is_called_by_not_buyer_then_deposit_is_not_returned(
        ) {
            let (mut escrow, _, buyer, seller) = create_contract(10, 99);
            assert_eq!(deposit(&mut escrow, buyer, 199, 10), Ok(()));

            advance_n_blocks(50);
            escrow.mark_contract_as_completed_when_time_passed();

            set_caller(seller);
            assert_eq!(escrow.refund_deposit(), Err(Error::FundsCannotBeReturned));
        }
    }
}
