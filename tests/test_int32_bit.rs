use co_digits_rs::{Int32, IntegerNumber, UInt32};

#[test]
fn test_int32_bit() {
    let number = Int32::from(-1);
    println!("{}", number.decode_twos_complement());
    println!("{}", number.decode_ones_complement());
    println!("{}", number.decode_original_code());

    let number = UInt32::from(1);
    println!("{}", number.decode_twos_complement());
    println!("{}", number.decode_ones_complement());
    println!("{}", number.decode_original_code());
}