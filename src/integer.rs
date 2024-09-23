use crate::IntegerNumber;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct UInt32 {
    pub(crate) bits: [bool; 32]
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Int32 {
    pub(crate) bits: [bool; 32]
}

impl UInt32 {
    pub fn new() -> Self {
        return Self {
            bits: [false; 32]
        }
    }
}

impl Int32 {
    pub fn new() -> Self {
        return Self {
            bits: [false; 32]
        }
    }
}

impl IntegerNumber for UInt32 {
    type Output = u32;
    type BinarySumOutput = u32;
    const BIT_SIZE: usize = 32;
    const BINARY_SUM_START_AT: usize = 0;

    fn is_negative(&self) -> bool {
        return false;
    }

    fn bits(&self) -> &[bool] {
        return &self.bits;
    }
}

impl IntegerNumber for Int32 {
    type Output = i32;
    type BinarySumOutput = u32;
    const BIT_SIZE: usize = 32;
    const BINARY_SUM_START_AT: usize = 1;
    
    fn is_negative(&self) -> bool {
        return self.bits[0];
    }

    fn bits(&self) -> &[bool] {
        return &self.bits;
    }
}

impl From<i32> for Int32 {
    fn from(value: i32) -> Self {
        let mut int32 = Self::new();
        
        let mut temp = value;

        for index in 0..=Self::BIT_SIZE - 1 {
            int32.bits[index] = temp & 1 != 0;
            temp >>= 1;
        }

        int32.bits.reverse();

        return int32;
    }
}

impl From<u32> for UInt32 {
    fn from(value: u32) -> Self {
        let mut uint32 = Self::new();
        let mut temp = value;

        for index in 0..=Self::BIT_SIZE - 1 {
            uint32.bits[index] = temp & 1 != 0;
            temp >>= 1;
        }

        uint32.bits.reverse();
        
        return uint32;
    }
}
