pub trait Number {
    fn bits(&self) -> &[bool];

    // 源码
    fn decode_original_code(&self) -> isize {
        let bits = self.bits();
        let is_negative: bool = bits[0];

        let result = number_binary_array(bits, 1);

        match isize::try_from(result) {
            Err(_) => panic!("overflow result: {}\n", result),
            Ok(number) => if is_negative {
                -number
            } else {
                number
            }
        }
        
    }

    // 反码
    fn decode_ones_complement(&self) -> isize {
        let bits = self.bits();
        let is_negative = bits[0];

        let decoded_bits = bits.iter().map(|bit| {
            if is_negative {
                !*bit
            } else {
                *bit
            }
        }).collect::<Vec<bool>>();

        let result = number_binary_array(&decoded_bits, 0);
        return match isize::try_from(result) {
            Err(_) => panic!("overflow result: {}\n", result),
            Ok(number) => if is_negative {
                -number
            } else {
                number
            }
        }
    }

    // 补码
    fn decode_twos_complement(&self) -> isize {
        let bits = self.bits();
        let is_negative = bits[0];

        let decoded_bits = bits.iter().map(|bit| {
            if is_negative {
                !*bit
            } else {
                *bit
            }
        }).collect::<Vec<bool>>();

        let result = number_binary_array(&decoded_bits, 0);
        return match isize::try_from(result) {
            Err(_) => panic!("overflow result: {}\n", result),
            Ok(number) => if is_negative {
                -number + 1
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
}

pub struct UInt32 {
    bits: [bool; 32]
}

pub struct Int32 {
    bits: [bool; 32]
}

impl Number for UInt32 {
    fn bits(&self) -> &[bool] {
        return &self.bits;
    }
}

impl Number for Int32 {
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

// 一个整数的n次幂
fn pow(x: usize, n: usize) -> usize {
    let mut result = 1;
    for _ in 1..=n {
        result *= x;
    }

    return result;
}

// 给定一串二进制数，从 start 计算其代表的数字
fn number_binary_array(bits: &[bool], start: usize) -> usize {
    let size = bits.len();
    let pows = (1..=size).rev().map(|n| {
        pow(2, n - 1) as usize
    }).collect::<Vec<usize>>();

    let pows = &pows[start..];
    let result = bits[start..=(size - 1)].iter()
        .zip(pows)
        .map(|(bit, pow)| {
            if *bit {
                *pow
            } else {
                0
            }
        }).fold(0, |r, x| r + x);

    return result;
}