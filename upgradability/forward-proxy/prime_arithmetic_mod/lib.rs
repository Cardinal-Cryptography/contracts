#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod prime_arithmetic_mod {
    use scale::{Decode, Encode};

    #[derive(Eq, PartialEq, Debug, Decode, Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        DivisibleByPrime,
    }

    #[ink(storage)]
    pub struct PrimeArithmeticMod {}

    impl PrimeArithmeticMod {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
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

    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn multiply_works() {
            let calc = PrimeArithmeticMod::new();
            assert_eq!(calc.multiply(123, 211, 113), 76);
        }

        #[ink::test]
        fn power_works() {
            let calc = PrimeArithmeticMod::new();
            assert_eq!(calc.power(8, 100, 13), 1);
        }

        #[ink::test]
        fn invert_works() {
            let calc = PrimeArithmeticMod::new();
            assert_eq!(calc.invert(14, 131), Ok(103));
        }

        #[ink::test]
        fn invert_errors_if_divisible() {
            let calc = PrimeArithmeticMod::new();
            assert_eq!(calc.invert(14, 7), Err(Error::DivisibleByPrime));
        }

        #[ink::test]
        fn divide_works() {
            let calc = PrimeArithmeticMod::new();
            assert_eq!(calc.divide(123, 211, 113), Ok(37));
        }
    }
}
