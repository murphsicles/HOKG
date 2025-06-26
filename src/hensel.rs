// src/hensel.rs

use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::error::Error;

use crate::utils::{mod_inverse, square_root_mod_p, gcd};

/// Lifts a seed point (x0, y0) on the curve y^2 = x^3 + ax + b mod p to mod p^k.
pub fn hensel_lift(
    p: &BigInt,
    a: &BigInt,
    b: &BigInt,
    x0: &BigInt,
    y0: &BigInt,
    k: usize,
) -> Result<(BigInt, BigInt), Box<dyn Error>> {
    // Verify seed point lies on the curve
    if (y0 * y0) % p != (x0 * x0 * x0 + a * x0 + b) % p {
        return Err("Seed point does not lie on the curve modulo p".into());
    }

    // Verify non-singularity
    let four = BigInt::from(4);
    let twenty_seven = BigInt::from(27);
    if (four * a * a * a + twenty_seven * b * b) % p == BigInt::zero() {
        return Err("Curve is singular modulo p".into());
    }

    let mut x_current = x0.clone();
    let mut y_current = y0.clone();
    let mut modulus = p.clone();

    for _ in 1..=k {
        modulus = &modulus * p;

        // Define f(x) = x^3 + ax + b - y^2
        let y_current_sq = &y_current * &y_current;
        let f = |x: &BigInt| -> BigInt {
            (x * x * x + a * x + b - &y_current_sq) % &modulus
        };
        let f_prime = |x: &BigInt| -> BigInt {
            (BigInt::from(3) * x * x + a) % &modulus
        };

        // Check if f'(x_current) is invertible modulo p
        if gcd(&f_prime(&x_current), p) != BigInt::one() {
            return Err("Derivative not invertible, lifting fails".into());
        }

        // Hensel's lemma: x_next = x_current - f(x_current) / f'(x_current)
        let f_val = f(&x_current);
        let f_prime_val = f_prime(&x_current);
        let inv = mod_inverse(&f_prime_val, &modulus)?;
        x_current = (&x_current - f_val * inv) % &modulus;

        // Update y using the curve equation: y^2 = x^3 + ax + b
        let y_square = (&x_current * &x_current * &x_current + a * &x_current + b) % &modulus;
        y_current = square_root_mod_p(&y_square, &modulus)
            .ok_or_else(|| "No valid square root for y_next".to_string())?;
    }

    Ok((x_current, y_current))
}
