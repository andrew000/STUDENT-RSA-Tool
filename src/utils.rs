use num_bigint_dig::BigUint;
use num_traits::{One, Zero};

// Function to convert Bytes Array to Integer
pub fn bytes_array_to_integer(message: &[u8]) -> BigUint {
    let mut grd: BigUint = BigUint::one();
    let mut num: BigUint = BigUint::zero();

    // Iterate over message characters and
    for char in message.iter() {
        num += char * &grd;
        grd *= BigUint::from(256_u32);
    }
    return num;
}

// Function to convert Integer to Bytes Array
pub fn integer_to_bytes_array(mut integer: BigUint) -> Vec<u8> {
    let mut message: Vec<u8> = Vec::new();

    while integer != BigUint::zero() {
        let module: BigUint = &integer % BigUint::from(256_u32);
        let character: u8 = module.to_str_radix(10).parse().unwrap();

        message.push(character);
        integer /= BigUint::from(256_u32);
    }
    return message;
}
