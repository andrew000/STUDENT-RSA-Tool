use num_bigint_dig::{BigInt, BigUint};
use num_traits::{Pow, Zero};

use crate::utils;

pub fn encrypt(message: &String, modulus: &BigUint, e: &BigUint) -> String {
    let message = message.clone().into_bytes();

    let mut encrypted_data: Vec<u8> = Vec::new();
    let mut step_size: u32 = 0;
    let mut tmp: BigUint = modulus / BigUint::from(256u32);
    let mut slice: &[u8];

    while tmp != BigUint::zero() {
        tmp /= BigUint::from(256u32);
        step_size += 1;
    }

    let cypher_len = Pow::pow(&BigInt::from(256_u32), step_size)
        .to_str_radix(10)
        .len()
        + 1;

    for i in (0..message.len()).step_by(step_size as usize) {
        if i + step_size as usize > message.len() {
            slice = &message[i..message.len()];
        } else {
            slice = &message[i..i + step_size as usize];
        }

        let integer_slice = utils::bytes_array_to_integer(slice);
        println!("integer_slice: {}", integer_slice);
        let mut cypher_text = integer_slice.modpow(&e, &modulus).to_str_radix(10);
        println!("cypher_text: {}", cypher_text);

        if cypher_text.len() < cypher_len {
            while cypher_len - cypher_text.len() > 0 {
                cypher_text = "0".to_string() + &cypher_text;
            }
        }

        encrypted_data.extend(cypher_text.into_bytes());
    }
    BigInt::parse_bytes(&encrypted_data, 10)
        .unwrap()
        .to_str_radix(16)
}
