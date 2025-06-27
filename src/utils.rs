// src/utils.rs

use num_bigint_dig::BigInt;
use num_traits::Zero;

// Computes the greatest common divisor (GCD) of two BigInt numbers using the Euclidean algorithm.
//
// # Arguments
// * `a` - First number.
// * `b` - Second number.
//
// # Returns
// The GCD of the two numbers as a BigInt.
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

// Computes the modular inverse of a number modulo m using the extended Euclidean algorithm.
//
// # Arguments
// * `a` - The number to find the inverse for.
// * `m` - The modulus.
//
// # Returns
// A `Result` containing the modular inverse of `a` modulo `m` or an error if it does not exist.
pub fn mod_inverse(a: &BigInt, m: &BigInt) -> Result<BigInt, Box<dyn std::error::Error>> {
    let (g, x, _) = extended_gcd(a, m);
    if g != BigInt::from(1) {
        return Err("Modular inverse does not exist".into());
    }
    Ok((x % m + m) % m)
}

// Helper function to compute the extended GCD of two BigInt numbers.
//
// # Arguments
// * `a` - First number.
// * `b` - Second number.
//
// # Returns
// A tuple `(g, x, y)` where `g` is the GCD and `x`, `y` satisfy ax + by = g.
fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    let mut old_r = a.clone();
    let mut r = b.clone();
    let mut old_x = BigInt::from(1);
    let mut x = BigInt::from(0);
    let mut old_y = BigInt::from(0);
    let mut y = BigInt::from(1);

    while !r.is_zero() {
        let quotient = &old_r / &r;
        let temp_r = r.clone();
        r = old_r - quotient.clone() * &r;
        old_r = temp_r;

        let temp_x = x.clone();
        x = old_x - quotient.clone() * &x;
        old_x = temp_x;

        let temp_y = y.clone();
        y = old_y - quotient * &y;
        old_y = temp_y;
    }

    (old_r, old_x, old_y)
}
