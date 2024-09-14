use co_digits_rs::{Int32, IntegerNumber, UInt32, Float32, Float64, FloatNumber};


fn main() {
    /* let number_1 = UInt32::from(2 as u32);
    let number_2 = Int32::from(1 as i32);
    let number_3 = Int32::from(-128 as i32);
    
    println!("original code: {}", number_1.decode_original_code());
    println!("ones complement: {}", number_1.decode_ones_complement());
    println!("twos complement: {}", number_1.decode_twos_complement());

    println!("debug, check the bits: {}", number_1.bits_string());

    println!("original code: {}", number_2.decode_original_code());
    println!("ones complement: {}", number_2.decode_ones_complement());
    println!("twos complement: {}", number_2.decode_twos_complement());

    println!("debug, check the bits: {}", number_2.bits_string());

    println!("original code: {}", number_3.decode_original_code());
    println!("ones complement: {}", number_3.decode_ones_complement());
    println!("twos complement: {}", number_3.decode_twos_complement());

    println!("debug, check the bits: {}", number_3.bits_string());
 */

    let float_1 = Float32::from(25.125 as f32);
    println!("float_1: {}", float_1.literal_value());
    println!("float_1: {}", float_1.bit_string());
    println!("float_1, exponent: {}", float_1.decode_exponent());
    println!("float_1, fraction: {}", float_1.decode_fraction());
}
