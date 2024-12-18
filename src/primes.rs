use num::{bigint::{RandBigInt, ToBigInt}, BigInt, BigUint, Zero};


//Handy macro to capsulate BigInt unwrapping
macro_rules! Bint {
    ($e: expr) => {
        ($e).to_bigint().unwrap()
    };
}

pub(crate) use Bint;


//Try go iterate bunch of random numbers until we hit 2 primes
pub fn get_primes() -> (BigInt, BigInt) {
    //Using pretty high number in case of bad luck, we exit immediately anyway
    let max_tries = 10000;
    let mut primes: Vec<BigInt> = Vec::new();

    for _ in 0..max_tries {
        let b1 = get_random_bigint();
        let b1_is_prime = is_prime(&b1);
        if b1_is_prime {
            primes.push(b1);

            if primes.len() == 2 {
                return (primes[0].clone(), primes[1].clone());
            }
        }
    }
    panic!("failed to find primes!");
}


pub fn is_prime(num: &BigInt) -> bool {
    
    //Some base-checks but in our case we do not really need these
    if num < &BigInt::from(2) {
        return false
    }

    //Check if inspected number is divisible by some of the first 500 primes --> is not prime
    let first_500_primes = get_500_first_primes();
    for prime in first_500_primes {
        let bprime = Bint!(prime);
        if num == &bprime {
            return true
        }
        if num % bprime == BigInt::zero() {
            return false;
        }
    }

    //Finally give number to Miller-Rabin
    return is_rb_prime(&num);
}

pub fn is_rb_prime(n: &BigInt) -> bool {
    let iterations: u16 = 600;
    let mut rng = rand::thread_rng();

    for _ in 0..iterations {

        // Get random base number we use to test if n is composite or not.
        let ubound = &sub_1(n.clone());
        let a = rng.gen_bigint_range(&Bint!(2), ubound);

        if is_composite(Bint!(a), &n) {
            return false
        }
    }   

    return true
}

fn sub_1(n: BigInt) -> BigInt {
    return &n - 1u8
}

// In Miller-Rabin test, we write any potential prime in following form: n = 1 + q*2^t
// We divide q until it becomes odd.
// By thgen, we have a correct number for "t" which is how many times 2 divides n-1(=q)
fn get_q_and_t(n: &BigInt) -> (BigInt, BigInt) {
    let mut q = sub_1(n.clone());
    let mut t: BigInt = Bint!(0);
    
    while q.clone() % Bint!(2) == Bint!(0) {
        t += 1;
        q /= 2;
    }

    return (q, t)

}


// In Miller-Rabin, we need to find if given "n" is composite and thus not a prime.
//"a" is used as a potential witness for "n".
fn is_composite(mut a: BigInt, n: &BigInt) -> bool {
    let (q, t) = get_q_and_t(n);
    let mut t = t;
    let n_minus_one = sub_1(n.clone());

    a = a.modpow(&q, n);

    //If a is 1, we know that n is not composite and can early return false
    if a == Bint!(1) {
        return false
    }

    // Perform loop for rest of the "t" values, and if we find that at any point "a" is equal to -1
    // we can conclude that "n" is a prime.
    while t > Bint!(0) {
        if a == n_minus_one {
            return false;
        }
        a = a.modpow(&Bint!(2), n);
        t = sub_1(t);
    }
    return true
}


fn get_random_bigint() -> BigInt {
    let mut rng = rand::thread_rng();
    let rand_num: BigUint = rng.gen_biguint(1024);
    return Bint!(rand_num);
}

//Use sieve 
fn get_500_first_primes() -> Vec<i32> {
    const MAX: usize = 500; 
    let mut cands: [bool; MAX] = [true; MAX]; 

    for p in 2..MAX {
        let p_multiple = p * p;
        if p_multiple >= MAX {
            break;
        }
        if cands[p] == true {
            for i in (p_multiple..MAX).step_by(p) {
                cands[i] = false;
            }
        }
    }

    let mut primes = Vec::with_capacity(500);
    for c in 2..MAX {
        if cands[c] {
            primes.push(c as i32); 
        }
    }
    return primes
}