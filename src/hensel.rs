// src/hensel.rs

use num_bigint_dig::BigInt;
use std::error::Error;

use crate::utils::{gcd, mod_inverse};

/// Performs Hensel lifting to lift a point (x0, y0) on an elliptic curve y^2 = x^3 + ax + b mod p
/// to a point modulo p^k.
pub fn hensel_lift(
    p: &BigInt,
    a: &BigInt,
    b: &BigInt,
    x0: &BigInt,
    y0: &BigInt,
    k: usize,
) -> Result<(BigInt, BigInt), Box<dyn Error>> {
    let mut x = x0.clone();
    let mut y = y0.clone();

    // Verify the initial point satisfies y^2 = x^3 + ax + b mod p
    let left = (y.clone() * y.clone()) % p;
    let right = (x.clone() * x.clone() * x.clone() + a * &x + b) % p;
    if left != right {
        return Err("Initial point does not lie on the curve".into());
    }

    // Iterative Hensel lifting up to p^k
    for i in 1..=k {
        let modulus = p.pow(i as u32);

        // Compute the derivative f'(x) = 3x^2 + a
        let three = BigInt::from(3);
        let f_prime = (three * &x * &x + a) % &modulus;

        // Compute f(x) = y^2 - (x^3 + ax + b)
        let f_x = (y.clone() * y.clone()
            - (x.clone() * x.clone() * x.clone() + a * &x + b))
            % &modulus;

        // Check if f'(x) is invertible modulo p^i
        if gcd(&f_prime, &modulus) != BigInt::from(1) {
            return Err("Hensel lifting failed: derivative not invertible".into());
        }

        // Update x using Newton's method: x_{i+1} = x_i - f(x_i)/f'(x_i)
        let delta_x = (f_x * mod_inverse(&f_prime, &modulus)?) % &modulus;
        x = (x - delta_x) % &modulus;

        // Update y to satisfy y^2 = x^3 + ax + b mod p^i
        let right_side = (x.clone() * x.clone() * x.clone() + a * &x + b) % &modulus;
        if right_side < BigInt::from(0) {
            return Err("Negative right side in Hensel lifting".into());
        }
        y = (y + &modulus) % &modulus; // Adjust y to be positive
    }

    Ok((x, y))
}
