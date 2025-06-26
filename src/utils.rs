// src/utils.rs

use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::error::Error;

/// Computes the modular inverse of `a` modulo `m` using the extended Euclidean algorithm.
pub fn mod_inverse(a: &BigInt, m: &BigInt) -> Result<BigInt, Box<dyn Error>> {
    let (g, x, _) = extended_gcd(a, m);
    if g != BigInt::one() {
        return Err("Modular inverse does not exist".into());
    }
    Ok((x % m + m) % m)
}

/// Extended Euclidean algorithm to compute GCD and Bézout coefficients.
fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if a.is_zero() {
        return (b.clone(), BigInt::zero(), BigInt::one());
    }
    let (g, x, y) = extended_gcd(&(b % a), a);
    (g, y - (b / a) * x.clone(), x)
}

/// Computes the square root of `value` modulo `p` (simplified for small primes).
/// Note: For production, use Tonelli-Shanks for general cases.
pub fn square_root_mod_p(value: &BigInt, p: &BigInt) -> Option<BigInt> {
    // Simplified: Try all residues for small primes
    let zero = BigInt::zero();
    let mut i = zero.clone();
    let p_minus_one = p - BigInt::one();
    while i <= p_minus_one {
        if (&i * &i) % p == value % p {
            return Some(i);
        }
        i += BigInt::one();
    }
    None
}

/// Computes GCD of two BigInts.
pub fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    let mut a = a.clone();
    let mut b = b.clone();
    while !b.is_zero() {
        let temp = b.clone();
        b = a % b;
        a = temp;
    }
    a
}
