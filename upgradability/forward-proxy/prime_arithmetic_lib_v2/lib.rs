#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod prime_arithmetic_lib_v2 {
    use ink_env::DefaultEnvironment;
    use scale::{Decode, Encode};

    #[derive(Eq, PartialEq, Debug, Decode, Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        DivisibleByPrime,
        PermissionDenied,
    }

    #[ink(storage)]
    pub struct PrimeArithmeticLibV2 {
        admin: AccountId,
    }

    impl PrimeArithmeticLibV2 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                admin: Self::env().caller(),
            }
        }

        /// Adds two numbers modulo a given prime
        #[ink(message)]
        pub fn add(&self, summand_1: u64, summand_2: u64, prime: u64) -> u64 {
            (((summand_1 as u128) + (summand_2 as u128)) % (prime as u128)) as u64
        }

        /// Multiplies two small numbers modulo a given prime
        #[ink(message)]
        pub fn multiply(&self, factor_1: u64, factor_2: u64, prime: u64) -> u64 {
            (((factor_1 as u128) * (factor_2 as u128)) % (prime as u128)) as u64
        }

        /// Performs fast exponentation modulo a given prime
        #[ink(message)]
        pub fn power(&self, base: u64, exponent: u64, prime: u64) -> u64 {
            let mut t_exp = exponent;
            let mut multiplicant = base;
            let mut result: u64 = 1;
            while t_exp > 0 {
                if t_exp % 2 == 1 {
                    result = self.multiply(result, multiplicant, prime);
                }
                t_exp /= 2;
                multiplicant = self.multiply(multiplicant, multiplicant, prime);
            }
            result
        }

        /// Inverts modulo a given prime
        #[ink(message)]
        pub fn invert(&self, number: u64, prime: u64) -> Result<u64, Error> {
            if number % prime == 0 {
                return Err(Error::DivisibleByPrime)
            }

            if prime == 2 {
                Ok(1)
            } else {
                Ok(self.power(number, prime-2, prime))
            }
        }

        /// Performs division modulo a given prime
        #[ink(message)]
        pub fn divide(&self, dividend: u64, divisor: u64, prime: u64) -> Result<u64, Error> {
            let divisor_inv = self.invert(divisor, prime)?;
            Ok(self.multiply(dividend, divisor_inv, prime))
        }

        /// Allows admin to terminate instance of this contract
        #[ink(message)]
        pub fn terminate(&mut self) -> Result<(), Error> {
            if self.env().caller() == self.admin {
                ink_env::terminate_contract::<DefaultEnvironment>(self.admin);
                // We do not return after calling terminate_contract
            } else {
                Err(Error::PermissionDenied)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn add_works() {
            let calc = PrimeArithmeticLibV2::new();
            assert_eq!(calc.add(2, 4, 5), 1);
        }

        #[ink::test]
        fn multiply_works() {
            let calc = PrimeArithmeticLibV2::new();
            assert_eq!(calc.multiply(123, 211, 113), 76);
        }

        #[ink::test]
        fn power_works() {
            let calc = PrimeArithmeticLibV2::new();
            assert_eq!(calc.power(8, 100, 13), 1);
        }

        #[ink::test]
        fn invert_works() {
            let calc = PrimeArithmeticLibV2::new();
            assert_eq!(calc.invert(14, 131), Ok(103));
        }

        #[ink::test]
        fn invert_errors_if_divisible() {
            let calc = PrimeArithmeticLibV2::new();
            assert_eq!(calc.invert(14, 7), Err(Error::DivisibleByPrime));
        }

        #[ink::test]
        fn divide_works() {
            let calc = PrimeArithmeticLibV2::new();
            assert_eq!(calc.divide(123, 211, 113), Ok(37));
        }
    }
}
