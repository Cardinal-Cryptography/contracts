#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod prime_arithmetic_mod_slow {
    use scale::{Decode, Encode};

    #[derive(Eq, PartialEq, Debug, Decode, Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        DivisibleByPrime,
    }

    #[ink(storage)]
    pub struct PrimeArithmeticModSlow {}

    impl PrimeArithmeticModSlow {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        /// Multiplies two small numbers modulo a given small prime
        #[ink(message)]
        pub fn multiply(&self, factor_1: u8, factor_2: u8, prime: u8) -> u8 {
            (((factor_1 as u16) * (factor_2 as u16)) % (prime as u16)) as u8
        }

        /// Performs exponentation modulo a given small prime
        #[ink(message)]
        pub fn power(&self, base: u8, exponent: u8, prime: u8) -> u8 {
            let mut result: u8 = 1;
            for _ in 0..exponent {
                result = self.multiply(result, base, prime);
            }
            result as u8
        }

        /// Inverts modulo a given small prime
        #[ink(message)]
        pub fn invert(&self, number: u8, prime: u8) -> Result<u8, Error> {
            for inv in 0..prime {
                if self.multiply(number, inv, prime) == 1 {
                    return Ok(inv);
                }
            }
            Err(Error::DivisibleByPrime)
        }

        /// Performs division modulo a given small prime
        #[ink(message)]
        pub fn divide(&self, dividend: u8, divisor: u8, prime: u8) -> Result<u8, Error> {
            let divisor_inv = self.invert(divisor, prime)?;
            Ok(self.multiply(dividend, divisor_inv, prime))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn multiply_works() {
            let calc = PrimeArithmeticModSlow::new();
            assert_eq!(calc.multiply(123, 211, 113), 76);
        }

        #[ink::test]
        fn power_works() {
            let calc = PrimeArithmeticModSlow::new();
            assert_eq!(calc.power(8, 100, 13), 1);
        }

        #[ink::test]
        fn invert_works() {
            let calc = PrimeArithmeticModSlow::new();
            assert_eq!(calc.invert(14, 131), Ok(103));
        }

        #[ink::test]
        fn invert_errors_if_divisible() {
            let calc = PrimeArithmeticModSlow::new();
            assert_eq!(calc.invert(14, 7), Err(Error::DivisibleByPrime));
        }

        #[ink::test]
        fn divide_works() {
            let calc = PrimeArithmeticModSlow::new();
            assert_eq!(calc.divide(123, 211, 113), Ok(37));
        }
    }
}
