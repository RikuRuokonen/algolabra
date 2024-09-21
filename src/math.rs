use num::BigInt;

pub fn euler_totient(q: BigInt, p: BigInt) -> BigInt {
    let result: BigInt = (q-1) * (p-1);
    return result
}

pub fn modular_multip_inverse(e: BigInt, totient: BigInt) {
 //TODO: Implement this last step of key generation
}