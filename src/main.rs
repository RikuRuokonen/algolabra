mod primes;
mod math;

use num::{bigint::{ToBigInt}};
use primes::Bint;
use std::env;



fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let (p1, p2) = primes::get_primes();
    println!("Found two primes: {p1} and  {p2}");
    let n = p1.clone() * p2.clone();
    println!("n is: {n}");
    let totient = math::euler_totient(p1.clone(), p2.clone());
    println!("totient is: {totient}");
    let e = Bint!(65537);
    let d = math::modular_multip_inverse(e, totient);
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
}