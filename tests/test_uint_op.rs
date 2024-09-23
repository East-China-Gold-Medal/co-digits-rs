use co_digits_rs::{OverflowException, UInt32};

#[test]
fn test_uint_op() {
    let left = UInt32::from(1 as u32);
    let right = UInt32::from(2 as u32);

    println!("{:?}", left + right);
    println!("{:?}", left - right);
    println!("{:?}", right - left);
}