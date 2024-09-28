mod primes;
mod math;

use base64::{engine::general_purpose, Engine as _};
use num::{bigint::ToBigInt, BigInt};
use num_bigint::Sign;
use primes::Bint;
use core::str;
use std::env;



fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let (public_key, private_key) = generate_key_pair();
    println!("Public Key: {:?}", public_key);
    println!("Private Key: {:?}", private_key);
    let original_message = "Test String";
    // Encrypt the message
    let encrypted_message = encrypt(&original_message, public_key);
    println!("Encyrpted message: {encrypted_message}");
    let decrypted_message = decrypt(&encrypted_message, private_key);
    println!("Decyrpted message: {decrypted_message}");

}

fn construct_public_key(n: BigInt, e: BigInt) -> (BigInt, BigInt) {
    return (n, e)
}

fn construct_private_key(n: BigInt, d: BigInt) -> (BigInt, BigInt) {
    return (n, d)
}

fn generate_key_pair() -> ((BigInt, BigInt), (BigInt, BigInt)){
    let (p1, p2) = primes::get_primes();
    let n = p1.clone() * p2.clone();
    let totient = math::euler_totient(p1.clone(), p2.clone());
    let e = Bint!(65537);
    let d = math::modular_multip_inverse(e.clone(), totient);
    let public_key = construct_public_key(n.clone(), e.clone());
    let private_key = construct_private_key(n.clone(), d.clone());
    return (public_key, private_key)
}


pub fn encrypt(message: &str, public_key: (BigInt, BigInt)) -> String {
    //TODO: Consider using BigUint across application.
    let encoded = BigInt::from_bytes_be(Sign::Plus, message.as_bytes());
    //Encrypt via modpow by using public key parts as exponent and modulus
    let encrypted = encoded.modpow(&public_key.1, &public_key.0);
    let encrypted_bytes = encrypted.to_bytes_be().1;
    let encrypted_base64 = general_purpose::STANDARD.encode(encrypted_bytes);
    return encrypted_base64
}

pub fn decrypt(ciphertext: &str, private_key: (BigInt, BigInt)) -> String {
    let decoded_bytes = general_purpose::STANDARD.decode(ciphertext).expect("Invalid Base64 conversion");
    let decoded = BigInt::from_bytes_be(Sign::Plus, &decoded_bytes);
    let decrypted = decoded.modpow(&private_key.1, &private_key.0);
    let decrypted_bytes = decrypted.to_bytes_be().1;
    return String::from_utf8(decrypted_bytes).expect("Invalid UTF-8 byte-representation");
}


//Initial tests for prime-gen checking
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_generation() {
        let some_known_primes: [i32; 5] = [11, 13, 443, 1289, 2027];
        for prime in some_known_primes {
            let is_p = primes::is_prime(&primes::Bint!(prime));
            assert_eq!(is_p, true);
         }
         let some_non_primes: [i32; 5] = [33, 68, 559, 1120, 4880];
         for non_prime in some_non_primes {
            let is_p = primes::is_prime(&primes::Bint!(non_prime));
            assert_eq!(is_p, false);
         }
    }
    #[test]
    fn test_encryption_decryption() {
        let (public_key, private_key) = generate_key_pair();
        let original_message = "Algolabra Test String";
        let encrypted_message = encrypt(original_message, public_key);
        let decrypted_message = decrypt(&encrypted_message, private_key);
        assert_eq!(original_message, decrypted_message, "The decrypted message does not match the original one.");
    }
}