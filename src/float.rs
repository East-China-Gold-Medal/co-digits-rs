use num_traits::checked_pow;

use crate::FloatNumber;

#[derive(Copy, Clone, Debug)]
pub struct Float32 {
    bits: [bool; 32]
}

#[derive(Copy, Clone, Debug)]
pub struct Float64 {
    bits: [bool; 64]
}

impl Float32 {
    fn new() -> Self {
        return Self {
            bits: [false; 32]
        }
    }
}

impl Float64 {
    fn new() -> Self {
        return Self {
            bits: [false; 64]
        }
    }
}

impl From<f32> for Float32 {
    fn from(value: f32) -> Self {
        let bits_u32: u32 = unsafe {
            std::mem::transmute(value)
        };
    
        let mut float32 = Self::new();
        let mut temp = bits_u32;
    
        for index in 0..=Self::BIT_SIZE - 1 {
            float32.bits[index] = temp & 1 != 0;
            temp >>= 1;
        }
    
        float32.bits.reverse();

        return float32;
    }
}

impl From<f64> for Float64 {
    fn from(value: f64) -> Self {
        let bits_u64: u64 = unsafe {
            std::mem::transmute(value)
        };

        let mut float64 = Self::new();
        let mut temp = bits_u64;

        for index in 0..=Self::BIT_SIZE - 1 {
            float64.bits[index] = temp & 1 != 0;
            temp >>= 1;
        }

        float64.bits.reverse();

        return float64;
    }
}

impl FloatNumber for Float32 {
    type Output = f32;
    const EXPONENT_SIZE: usize = 8;
    const FRACTION_SIZE: usize = 23;
    const BIAS: i32 = -127;
    const BIT_SIZE: usize = 32;

    fn bits(&self) -> &[bool] {
        return &self.bits;
    }

    fn decode_exponent(&self) -> i32 {
        let pows = (1..=Self::EXPONENT_SIZE).rev().map(|n| {
             match checked_pow(2, n - 1) {
                None => panic!("overflow when calculating 2^{}", n - 1),
                Some(number) => number
             }
        }).collect::<Vec<_>>();

        return self.exponent_bits().iter()
            .zip(pows)
            .map(|(bit, pow)| {
                if *bit {
                    pow
                } else {
                    0
                }
            }).fold(Self::BIAS, |r, x| r + x);
    }

    fn decode_fraction(&self) -> Self::Output {
        let pows = (1..=Self::FRACTION_SIZE - 1).map(|n| {
            f32::powf(0.5, n as f32)
        });

        return self.fraction_bits().iter()
            .zip(pows)
            .map(|(bit, pow)| {
                if *bit {
                    pow
                } else {
                    0.0
                }
            }).fold(1.0, |r, x| r + x);
    }

    fn literal_value(&self) -> Self::Output {
        let is_negative = self.sign_bit();
        let exponent = self.decode_exponent();
        let fraction = self.decode_fraction();

        let result = f32::powi(2.0, exponent) * fraction;
        return if is_negative {
            -result
        } else {
            result
        }
    }
}

impl FloatNumber for Float64 {
    type Output = f64;
    const EXPONENT_SIZE: usize = 11;
    const FRACTION_SIZE: usize = 52;
    const BIAS: i32 = -1023;
    const BIT_SIZE: usize = 32;

    fn bits(&self) -> &[bool] {
        return &self.bits;
    }
    
    fn decode_exponent(&self) -> i32 {
        let pows = (1..=Self::EXPONENT_SIZE).rev().map(|n| {
             match checked_pow(2, n - 1) {
                None => panic!("overflow when calculating 2^{}", n - 1),
                Some(number) => number
             }
        }).collect::<Vec<_>>();

        return self.exponent_bits().iter()
            .zip(pows)
            .map(|(bit, pow)| {
                if *bit {
                    pow
                } else {
                    0
                }
            }).fold(Self::BIAS, |r, x| r + x);
    }

    fn decode_fraction(&self) -> Self::Output {
        let pows = (1..=Self::FRACTION_SIZE - 1).map(|n| {
            f64::powf(0.5, n as f64)
        });

        return self.fraction_bits().iter()
            .zip(pows)
            .map(|(bit, pow)| {
                if *bit {
                    pow
                } else {
                    0.0
                }
            }).fold(1.0, |r, x| r + x);
    }

    fn literal_value(&self) -> Self::Output {
        let is_negative = self.sign_bit();
        let exponent = self.decode_exponent();
        let fraction = self.decode_fraction();

        let result = f64::powi(2.0, exponent) * fraction;
        return if is_negative {
            -result
        } else {
            result
        }
    }
}