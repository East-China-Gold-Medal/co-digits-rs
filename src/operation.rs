use crate::{integer::{Int32, UInt32}, IntegerNumber};
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::Ord;
use crate::OverflowException;


impl Add for UInt32 {
    type Output = Result<Self, OverflowException>;

    fn add(self, other: Self) -> Self::Output {
        let mut result = Self::new();
        let mut carry = false;
        for index in (Self::BINARY_SUM_START_AT..=Self::BIT_SIZE - 1).rev() {
            let left = self.bits()[index];
            let right = other.bits()[index];
            
            result.bits[index] = left ^ right ^ carry;
            carry = (left && right) || (right && carry);
        }
        
        return if carry {
            Err(OverflowException)
        } else {
            Ok(result)
        };
    }
}

impl Neg for UInt32 {
    type Output = [bool; Self::BIT_SIZE];

    fn neg(self) -> Self::Output {
        let mut result = [false; Self::BIT_SIZE];
        for (index, bit) in self.bits().iter().enumerate() {
            result[index] = !*bit;
        }

        let mut carry = true;
        for index in (Self::BINARY_SUM_START_AT..=Self::BIT_SIZE - 1).rev() {
            result[index] = result[index] ^ carry;
            carry = !result[index] && carry;
        }
        
        return result;   
    }
}

impl Sub for UInt32 {
    type Output = Result<Self, OverflowException>;

    fn sub(self, other: Self) -> Self::Output {
        let negative = other.neg();
        let mut result = Self::new();
        let mut carry = false;
        for index in (0..=Self::BIT_SIZE - 1).rev() {
            let left = self.bits()[index];
            let right = negative[index];
            
            result.bits[index] = left ^ right ^ carry;
            carry = (left && right) || (right && carry);
        }
        
        return if result > self {
            Err(OverflowException)
        } else {
            Ok(result)
        };
    }
}

impl PartialOrd for UInt32 {
    fn le(&self, other: &Self) -> bool {
        return self.decode_original_code() <= other.decode_original_code();
    }

    fn ge(&self, other: &Self) -> bool {
        return self.decode_original_code() >= other.decode_original_code();
    }

    fn gt(&self, other: &Self) -> bool {
        return self.decode_original_code() > other.decode_original_code();
    }

    fn lt(&self, other: &Self) -> bool {
        return self.decode_original_code() < other.decode_original_code();
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UInt32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.decode_original_code().cmp(&other.decode_original_code());
    }
}

impl Add for Int32 {
    type Output = Result<Self, OverflowException>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();

        let mut carry = false;
        for index in (Self::BINARY_SUM_START_AT..=Self::BIT_SIZE - 1).rev() {
            let left = self.bits()[index];
            let right = rhs.bits()[index];

            result.bits[index] = left ^ right ^ carry;

            carry = (left && right) || (right && carry);
        }

        return if carry {
            Err(OverflowException)
        } else {
            Ok(result)
        };
    }
}

impl Neg for Int32 {
    type Output = [bool; Self::BIT_SIZE];

    fn neg(self) -> Self::Output {
        let mut result = [false; Self::BIT_SIZE];
        for (index, bit) in self.bits().iter().enumerate() {
            result[index] = !*bit;
        }

        let mut carry = true;
        for index in (Self::BINARY_SUM_START_AT..=Self::BIT_SIZE - 1).rev() {
            result[index] = result[index] ^ carry;
            carry= !result[index] && carry;
        }

        return result;
    }
}

impl Sub for Int32 {
    type Output = Result<Self, OverflowException>;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();
        let negative = rhs.neg();

        let mut carry = false;
        for index in (0..=Self::BIT_SIZE - 1).rev() {
            let left = self.bits()[index];
            let right = negative[index];
            
            result.bits[index] = left ^ right ^ carry;
            carry = (left && right) || (right && carry);
        }
        
        return if carry {
            Err(OverflowException)
        } else {
            Ok(result)
        };
    }
}

impl PartialOrd for Int32 {
    fn le(&self, other: &Self) -> bool {
        return self.decode_original_code() <= other.decode_original_code();
    }

    fn ge(&self, other: &Self) -> bool {
        return self.decode_original_code() >= other.decode_original_code();
    }

    fn gt(&self, other: &Self) -> bool {
        return self.decode_original_code() > other.decode_original_code();
    }

    fn lt(&self, other: &Self) -> bool {
        return self.decode_original_code() < other.decode_original_code();
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Int32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.decode_original_code().cmp(&other.decode_original_code());
    }
}