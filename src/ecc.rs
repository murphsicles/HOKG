// src/ecc.rs

use crate::point::Point;
use num_bigint_dig::BigInt;
use num_traits::Zero;

/// Provides functionality for elliptic curve operations, specifically point multiplication.
///
/// This module implements the core arithmetic for elliptic curve cryptography,
/// including point addition and doubling over a finite field.
pub mod ecc {
    /// Performs elliptic curve point multiplication to compute the public key.
    ///
    /// This function implements the double-and-add algorithm to multiply a point on an
    /// elliptic curve by a scalar, producing a new point (the public key) in the finite
    /// field defined by `modulus`. The curve is specified by the coefficient `a`, while
    /// the `b` coefficient is included for interface consistency but not used.
    ///
    /// # Arguments
    /// * `scalar` - The private key scalar to multiply the base point by.
    /// * `point` - The base point on the elliptic curve.
    /// * `a` - The 'a' coefficient of the elliptic curve equation y^2 = x^3 + ax + b.
    /// * `_b` - The 'b' coefficient of the elliptic curve (unused but kept for interface consistency).
    /// * `modulus` - The modulus defining the finite field.
    ///
    /// # Returns
    /// A `Result` containing the resulting point (the public key) or an error if the
    /// modular inverse computation fails.
    ///
    /// # Errors
    /// Returns an error if the modular inverse cannot be computed (e.g., if the denominator
    /// is not invertible modulo `modulus`).
    pub fn elliptic_curve_multiply(
        scalar: &BigInt,
        point: &Point,
        a: &BigInt,
        _b: &BigInt,
        modulus: &BigInt,
    ) -> Result<Point, Box<dyn std::error::Error>> {
        // Initialize result as the point at infinity
        let mut result = Point::Infinity;
        // Create a mutable copy of the input point for iterative doubling
        let mut temp = point.clone();
        // Get binary representation of the scalar for double-and-add algorithm
        let scalar_bits = scalar.to_bytes_be().1;

        // Iterate over each bit of the scalar in reverse order
        for bit in scalar_bits.iter().rev() {
            // Process each bit from most significant to least significant
            for i in (0..8).rev() {
                if (bit >> i) & 1 == 1 {
                    // Perform point addition if the bit is 1
                    result = match (result.clone(), temp.clone()) {
                        (Point::Infinity, p) | (p, Point::Infinity) => p,
                        (Point::Coordinates(x1, y1), Point::Coordinates(x2, y2)) => {
                            if x1 == x2 && y1 == -&y2 {
                                // Points are inverses, result is point at infinity
                                Point::Infinity
                            } else {
                                // Compute slope for point addition or doubling
                                let slope = if x1 == x2 && y1 == y2 {
                                    // Point doubling: slope = (3x^2 + a) / (2y)
                                    let numerator = BigInt::from(3) * &x1 * &x1 + a;
                                    let denominator = BigInt::from(2) * &y1;
                                    let inv = crate::mod_inverse(&denominator, modulus)?;
                                    (numerator * inv) % modulus
                                } else {
                                    // Point addition: slope = (y2 - y1) / (x2 - x1)
                                    let numerator = &y2 - &y1;
                                    let denominator = &x2 - &x1;
                                    let inv = crate::mod_inverse(&denominator, modulus)?;
                                    (numerator * inv) % modulus
                                };

                                // Compute new coordinates
                                let x3 = (&slope * &slope - &x1 - &x2) % modulus;
                                let y3 = (&slope * (&x1 - &x3) - &y1) % modulus;
                                Point::Coordinates(x3, y3)
                            }
                        }
                    };
                }
                // Perform point doubling for each bit
                temp = match temp {
                    Point::Infinity => Point::Infinity,
                    Point::Coordinates(x, y) => {
                        if y.is_zero() {
                            // If y-coordinate is zero, result is point at infinity
                            Point::Infinity
                        } else {
                            // Compute slope for doubling: slope = (3x^2 + a) / (2y)
                            let slope = {
                                let numerator = BigInt::from(3) * &x * &x + a;
                                let denominator = BigInt::from(2) * &y;
                                let inv = crate::mod_inverse(&denominator, modulus)?;
                                (numerator * inv) % modulus
                            };
                            // Compute new coordinates
                            let x3 = (&slope * &slope - &x - &x) % modulus;
                            let y3 = (&slope * (&x - &x3) - &y) % modulus;
                            Point::Coordinates(x3, y3)
                        }
                    }
                };
            }
        }

        Ok(result)
    }
}
