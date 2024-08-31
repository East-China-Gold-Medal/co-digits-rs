#[test]
fn test_128() {
    let pow_127 = pow(2, 127);
    
    println!("{:?}", u128::try_from(pow_127));
}

fn pow(number: u128, n: usize) -> u128 {
    let mut result: u128 = 1;
    for _ in 1..=n {
        result *= number;
    }

    return result;
}