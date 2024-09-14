use std::fmt::Display;
use num_traits::{checked_pow, CheckedNeg, Float, PrimInt};

mod integer;
mod float;
pub use integer::*;
pub use float::*;

pub trait IntegerNumber {
    type BinarySumOutput: PrimInt + TryFrom<u32> + From<u32> + Display;
    type Output: PrimInt + TryFrom<Self::BinarySumOutput> + CheckedNeg;

    const BIT_SIZE: usize;
    const BINARY_SUM_START_AT: usize;

    fn bits(&self) -> &[bool];
    
    
    fn is_negative(&self) -> bool;

    fn decode_original_code(&self) -> Self::Output {
        let bits = self.bits();
        let result = Self::number_binary_array(bits, Self::BINARY_SUM_START_AT);
        match Self::Output::try_from(result) {
            Err(_) => panic!("overflow when parse {} to Output", result),
            Ok(number) => if self.is_negative() {
                number.checked_neg().expect("error when using unary")
            } else {
                number
            }
        }
        
    }

    fn decode_ones_complement(&self) -> Self::Output {
        let bits = self.bits();
        let result = Self::number_binary_array(bits, Self::BINARY_SUM_START_AT);
        match Self::Output::try_from(result) {
            Err(_) => panic!("overflow when parse {} to Output", result),
            Ok(number) => if self.is_negative() {
                number.checked_neg().expect("error when using unary")
            } else {
                number
            }
        }
    }

    fn decode_twos_complement(&self) -> Self::Output {
        let bits = self.bits();
        let result = Self::number_binary_array(bits, Self::BINARY_SUM_START_AT);
        match Self::Output::try_from(result) {
            Err(_) => panic!("overflow when parse {} to Output", result),
            Ok(number) => if self.is_negative() {
                number.checked_neg().expect("error when using unary")
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

    fn number_binary_array(bits: &[bool], start: usize) -> Self::BinarySumOutput {
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
        let result = bits[start..=(size - 1)].iter()
            .zip(pows)
            .map(|(bit, pow)| {
                if *bit {
                    *pow
                } else {
                    Self::BinarySumOutput::from(0)
                }
            }).fold(Self::BinarySumOutput::from(0), |r, x| r + x);
    
        return result;
    }
}

pub trait FloatNumber {
    type Output: Float;
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
}
