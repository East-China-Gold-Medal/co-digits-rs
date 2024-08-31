use std::fmt::Display;
use num_traits::{checked_pow, CheckedNeg, PrimInt};

pub trait Number {
    type BinarySumOutput: PrimInt + TryFrom<u32> + From<u32> + Display;
    type Output: PrimInt + TryFrom<Self::BinarySumOutput> + CheckedNeg;

    fn bits(&self) -> &[bool];
    fn binary_sum_start_at(&self) -> usize;
    fn is_negative(&self) -> bool;

    fn decode_original_code(&self) -> Self::Output {
        let bits = self.bits();
        let result = Self::number_binary_array(bits, self.binary_sum_start_at());
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
        let result = Self::number_binary_array(bits, self.binary_sum_start_at());
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
        let result = Self::number_binary_array(bits, self.binary_sum_start_at());
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

        return result;
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

pub struct UInt32 {
    bits: [bool; 32]
}

pub struct Int32 {
    bits: [bool; 32]
}

impl Number for UInt32 {
    type Output = u32;
    type BinarySumOutput = u32;

    fn is_negative(&self) -> bool {
        return false;
    }
    
    fn binary_sum_start_at(&self) -> usize {
        return 0;
    }

    fn bits(&self) -> &[bool] {
        return &self.bits;
    }
}

impl Number for Int32 {
    type Output = i32;
    type BinarySumOutput = u32;

    fn is_negative(&self) -> bool {
        return self.bits[0];
    }

    fn binary_sum_start_at(&self) -> usize {
        return 1;
    }

    fn bits(&self) -> &[bool] {
        return &self.bits;
    }
}

// TODO implement From trait for UInt32 and Int32
impl From<i32> for Int32 {
    fn from(value: i32) -> Self {
        let mut bits = [false; 32];
        let mut temp = value;

        for index in 0..=31 {
            bits[index] = temp & 1 != 0;
            temp >>= 1;
        }

        bits.reverse();

        return Self {
            bits
        }
    }
}

impl From<u32> for UInt32 {
    fn from(value: u32) -> Self {
        let mut bits = [false; 32];
        let mut temp = value;

        for index in 0..31 {
            bits[index] = temp & 1 != 0;
            temp >>= 1;
        }

        bits.reverse();
        
        return Self {
            bits
        }
    }
}
