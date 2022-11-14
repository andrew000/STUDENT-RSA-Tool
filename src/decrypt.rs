use num_bigint_dig::{BigInt, BigUint};
use num_traits::{Pow, Zero};
use std::string::FromUtf8Error;

use crate::utils;

pub fn decrypt(message: &String, modulus: &BigUint, d: &BigInt) -> Result<String, FromUtf8Error> {
    let binding: String = BigUint::parse_bytes(message.as_bytes(), 16)
        .unwrap()
        .to_str_radix(10);
    let mut message: Vec<u8> = binding.as_bytes().to_vec();
    let mut decrypted_data: Vec<u8> = Vec::new();
    let mut step_size: u32 = 0;
    let mut tmp: BigUint = modulus / BigUint::from(256u32);

    let mut slice: &[u8];

    while tmp != BigUint::zero() {
        tmp /= BigUint::from(256u32);
        step_size += 1;
    }

    let cypher_len = Pow::pow(&BigUint::from(256_u32), step_size)
        .to_str_radix(10)
        .len()
        + 1;

    while message.len() % cypher_len != 0 {
        // Concatenate two slices
        message.reverse();
        message.push("0".as_bytes()[0]);
        message.reverse();
    }

    for i in (0..message.len()).step_by(cypher_len as usize) {
        if i + step_size as usize > message.len() {
            slice = &message[i..message.len()];
        } else {
            slice = &message[i..i + cypher_len as usize];
        }
        let integer_slice = BigUint::parse_bytes(slice, 10).unwrap();

        let convert_to_vec_string = &utils::integer_to_bytes_array(
            integer_slice.modpow(&d.to_biguint().unwrap(), &modulus),
        );
        decrypted_data.extend(convert_to_vec_string);
    }

    return String::from_utf8(decrypted_data);
}
