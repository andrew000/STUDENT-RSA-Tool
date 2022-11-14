use num_bigint_dig::{BigInt, BigUint, ModInverse};
use num_primes::Generator;
use num_traits::One;

use crate::decrypt::decrypt;
use crate::encrypt::encrypt;
use crate::key_storage::KeyPair;

const EXPONENT: u64 = 65537;

pub fn generate_primes(key_size: usize) -> (BigUint, BigUint) {
    let p = Generator::new_prime(key_size);
    let p = BigUint::from_radix_be(&p.to_radix_be(16), 16).unwrap();

    let q = Generator::new_prime(key_size);
    let q = BigUint::from_radix_be(&q.to_radix_be(16), 16).unwrap();
    (p, q)
}

pub fn generate_key_pair(key_size: usize) -> KeyPair {
    loop {
        let exponent: BigUint = BigUint::from(EXPONENT);

        let (p, q) = generate_primes(key_size);
        let modulus: BigUint = &p * &q;
        let phi: BigUint = (&p - BigUint::one()) * (&q - BigUint::one());
        let d: BigInt = (&exponent).mod_inverse(&phi).unwrap();
        let key_pair = KeyPair::new("default".to_string(), p, q, modulus, exponent, d);

        if test_key_pair(&key_pair) == true {
            return key_pair;
        }
    }
}

fn test_key_pair(key_pair: &KeyPair) -> bool {
    let test_messages = vec![
        "0".to_string(),
        ".".to_string(),
        "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .to_string(),
    ];
    for test_message in test_messages.iter() {
        let encrypted = encrypt(test_message, &key_pair.modulus, &key_pair.e);
        let decrypted = decrypt(&encrypted, &key_pair.modulus, &key_pair.d);

        if decrypted.is_err() || &decrypted.unwrap() != test_message {
            false;
        }
    }
    true
}
