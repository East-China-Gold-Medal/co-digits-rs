#[test]
fn test_binary_float() {
    let number = 3.456f32;

    let bits_u32: u32 = unsafe {
        std::mem::transmute(number)
    };

    let mut bits = [false; 32];
    let mut temp = bits_u32;

    for index in 0..31 {
        bits[index] = temp & 1 != 0;
        temp >>= 1;
    }

    bits.reverse();

    let sign_bit = bits[0];
    let exponent_bits = &bits[1..=8];
    let fraction_bits = &bits[9..];

    let map_fn = |bit: &bool| {
        return if *bit {
            1
        } else {
            0
        }
    };

    println!("sign_bit: {}", map_fn(&sign_bit));
    println!("exponent_bits: {}", exponent_bits.iter().map(map_fn).fold(String::new(), |r, x| format!("{}{}", r, x)));
    println!("fraction_bits: {}", fraction_bits.iter().map(map_fn).fold(String::new(), |r, x| format!("{}{}", r, x)));
    
}