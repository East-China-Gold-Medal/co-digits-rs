use std::fmt::Display;
use num_traits::{checked_pow, CheckedNeg, Float, PrimInt};

mod integer;
mod float;
mod operation;
mod error;
mod extension;

pub use integer::*;
pub use float::*;
pub use error::*;
pub use extension::*;


pub trait IntegerNumber {
    type BinarySumOutput: PrimInt + TryFrom<u32> + From<u32> + Display;
    type Output: PrimInt + TryFrom<Self::BinarySumOutput> + TryFrom<Self::Output> + CheckedNeg;
    const BIT_SIZE: usize;
    const BINARY_SUM_START_AT: usize;

    fn bits(&self) -> &[bool];
    
    fn is_negative(&self) -> bool;

    // 原码
    fn decode_original_code(&self) -> Self::Output {
        let bits = if self.is_negative() {
            self.bits().iter().map(|bit| {
                !*bit
            }).collect()
        } else {
            Vec::from(self.bits())
        };

        let result = Self::number_binary_array(bits.iter(), Self::BINARY_SUM_START_AT);
        match Self::Output::try_from(result) {
            Err(_) => panic!("overflow when parse {} to Output", result),
            Ok(number) => if self.is_negative() {
                number.checked_neg().expect("error when using unary")
            } else {
                number
            }
        }
        
    }

    // 反码
    fn decode_ones_complement(&self) -> Self::Output {
        let bits = if self.is_negative() {
            self.bits().iter().map(|bit| {
                !*bit
            }).collect()
        } else {
            Vec::from(self.bits())
        };

        let result = Self::number_binary_array(bits.iter(), Self::BINARY_SUM_START_AT);
        match Self::Output::try_from(result) {
            Err(_) => panic!("overflow when parse {} to Output", result),
            Ok(number) => if self.is_negative() {
                number.checked_neg().expect("error when using unary")
            } else {
                number
            }
        }
    }

    // 补码
    fn decode_twos_complement(&self) -> Self::Output {
        let bits = if self.is_negative() {
            self.bits().iter().map(|bit| {
                !*bit
            }).collect::<Vec<bool>>()
        } else {
            Vec::from(self.bits())
        };
        
        let result = Self::number_binary_array(bits.iter(), Self::BINARY_SUM_START_AT);
        match Self::Output::try_from(result) {
            Err(_) => panic!("overflow when parse {} to Output", result),
            Ok(number) => if self.is_negative() {
                /* let neg = number.checked_neg().expect("error when using unary");
                increment_one(neg) */

                increment_one(number).checked_neg().expect("error when using unary")
            } else {
                number
            }
        }
    }

    fn bits_string(&self) -> String {
        let result = self.bits().iter().map(|bit| {
            if *bit {
                1
            } else {
                0
            }
        }).fold(String::new(), |r, x| format!("{}{}", r, x));

        let mut grouped = String::new();
        
        for (index, ch) in result.chars().enumerate() {
            if index % 8 == 0  && index != 0 {
                grouped += ",";
            }

            grouped += ch.to_string().as_ref();
        }
        return grouped;
    }

    fn number_binary_array<'a, T>(bits: T, start: usize) -> Self::BinarySumOutput 
    where T: Iterator<Item = &'a bool> + {
        let bits = bits.collect::<Vec<_>>();
        let size = bits.len();
        let pows = (1..=size).rev().map(|n| {
            let two = Self::BinarySumOutput::try_from(2);
            match two {
                Err(_) => panic!("error when parse 2"),
                Ok(two) => match checked_pow(two, n - 1) {
                    None => panic!("overflow calculating 2^{}", n - 1),
                    Some(number) => number
                }
            }
            
        }).collect::<Vec<_>>();
    
        let pows = &pows[start..];
        let result = bits[start..].iter()
            .zip(pows)
            .map(|(bit, pow)| {
                if **bit {
                    *pow
                } else {
                    Self::BinarySumOutput::from(0)
                }
            }).fold(Self::BinarySumOutput::from(0), |r, x| r + x);
    
        return result;
    }
}

pub trait FloatNumber {
    type Output: Float + TryFrom<Self::Output> + From<Self::Output>;
    const EXPONENT_SIZE: usize;
    const FRACTION_SIZE: usize;
    const BIT_SIZE: usize;
    const BIAS: i32;
    fn bits(&self) -> &[bool];
    
    fn sign_bit(&self) -> bool {
        return self.bits()[0];
    }

    // 阶码
    fn exponent_bits(&self) -> &[bool] {
        return &self.bits()[1..=Self::EXPONENT_SIZE];
    }

    fn min_exponent_value() -> i32 {
        let min_exponent = 0;
        return min_exponent + Self::BIAS;
    }

    // 尾数
    fn fraction_bits(&self) -> &[bool] {
        return &self.bits()[Self::EXPONENT_SIZE + 1..=Self::BIT_SIZE - 1];
    }

    fn decode_exponent(&self) -> i32;
    fn decode_fraction(&self) -> Self::Output;
    fn literal_value(&self) -> Self::Output;

    fn bit_string(&self) -> String {
        let result = self.bits().iter().map(|bit| {
            if *bit {
                1
            } else {
                0
            }
        }).fold(String::new(), |r, x| format!("{}{}", r, x));

        let mut grouped = String::new();
        
        for (index, ch) in result.chars().enumerate() {
            grouped += ch.to_string().as_ref();
            if index == 0 {
                grouped += " ";
            }

            if index == Self::EXPONENT_SIZE {
                grouped += " "
            }
            
        }

        return grouped;
    }

    fn is_zero(&self) -> bool {
        return self.decode_exponent() < Self::min_exponent_value();
    }

    fn is_nan(&self) -> bool {
        return self.exponent_bits().iter().all(|bit| *bit) && self.fraction_bits().iter().any(|bit| *bit);
    }

    fn is_inf(&self) -> bool {
        return self.exponent_bits().iter().all(|bit| *bit) && self.fraction_bits().iter().any(|bit| !*bit);
    }

    fn abs_min_value() -> Self::Output {
        
        todo!()
    }

    fn abs_max_value() -> Self::Output {
        
        todo!()
    }
}
