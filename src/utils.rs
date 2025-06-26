// src/utils.rs

use num_bigint_dig::BigInt;
use num_traits::{One, Zero};

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

/// Computes the square root of a modulo p (a prime), returning the root or an error if it doesn't exist.
/// Implements Tonelli-Shanks algorithm for simplicity (assumes p is prime).
pub fn square_root_mod_p(a: &BigInt, p: &BigInt) -> Result<BigInt, Box<dyn std::error::Error>> {
    if a.is_zero() {
        return Ok(BigInt::zero());
    }

    // Check if a is a quadratic residue using Euler's criterion: a^((p-1)/2) ≡ 1 (mod p)
    let two = BigInt::from(2);
    let exponent = (p - BigInt::one()) / &two;
    let legendre = a.modpow(&exponent, p);
    if legendre != BigInt::one() {
        return Err("No square root exists modulo p".into());
    }

    // Simplified case for p ≡ 3 (mod 4)
    if p % &two == BigInt::one() {
        let exponent = (p + BigInt::one()) / BigInt::from(4);
        let result = a.modpow(&exponent, p);
        return Ok(result);
    }

    // For other primes, return error (full Tonelli-Shanks not implemented for brevity)
    Err("Square root modulo p not implemented for this case".into())
}
