// src/utils.rs

use num_bigint_dig::traits::{One, Zero};
use num_bigint_dig::BigInt;

/// Computes the greatest common divisor of a and b using the extended Euclidean algorithm,
/// returning the GCD.
pub fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    let (mut x, mut y) = (a.clone(), b.clone());
    while !y.is_zero() {
        let temp = y.clone();
        y = x % y;
        x = temp;
    }
    x
}

/// Computes the modular inverse of a modulo m using the extended Euclidean algorithm,
/// returning the inverse or an error if it doesn't exist.
pub fn mod_inverse(a: &BigInt, m: &BigInt) -> Result<BigInt, Box<dyn std::error::Error>> {
    let (mut a, mut m) = (a.clone(), m.clone());
    let (mut x, mut y) = (BigInt::zero(), BigInt::one());
    let (mut x_prev, mut y_prev) = (BigInt::one(), BigInt::zero());

    while !m.is_zero() {
        let quotient = &a / &m;
        let temp = m.clone();
        m = a % m;
        a = temp;

        let temp_x = x.clone();
        x = x_prev - quotient * &x;
        x_prev = temp_x;

        let temp_y = y.clone();
        y = y_prev - quotient * &y;
        y_prev = temp_y;
    }

    if a != BigInt::one() {
        return Err("Modular inverse does not exist".into());
    }

    // Ensure the result is positive
    Ok((x_prev % m + m) % m)
}
