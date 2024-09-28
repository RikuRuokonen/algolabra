use num::{one, BigInt, One, Zero};

pub fn euler_totient(q: BigInt, p: BigInt) -> BigInt {
    let result: BigInt = (q-1) * (p-1);
    return result
}

// Find (e⋅d)≡1 (mod totient) with extended euclidean algorithm
pub fn modular_multip_inverse(e_param: BigInt, totient_param: BigInt) -> BigInt {
    let mut s: BigInt = BigInt::zero();
    //This old_s will house modular inverso 
    let mut old_s = BigInt::one();
    let mut t: BigInt = BigInt::one();
    let mut old_t = BigInt::zero();
    let mut r: BigInt = totient_param.clone();
    let mut old_r = e_param.clone();

    while r > BigInt::zero() {
        let quotient = old_r.clone() / r.clone();

        //Tuple switch:  (old_r, r) <- (r, old_r - quotient * r)
        let temp_r = r.clone();
        r = old_r.clone() - quotient.clone() * r.clone();
        old_r = temp_r;

        //Similar tuple-switch for s
        let temp_s = s.clone();
        s = old_s.clone() - quotient.clone() * s.clone(); 
        old_s = temp_s;

        //Similar tuple-switch for t
        let temp_t = t.clone();
        t = old_t.clone() - quotient.clone() * t.clone(); 
        old_t = temp_t;
    }

    // Ensure the result is positive
    let result = old_s % &totient_param;
    // If the result is negative, perform adjustment by adding totient-param
    if result < BigInt::zero() {
        return result + &totient_param;
    }
    result


}

