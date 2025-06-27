// src/ecc.rs

use crate::point::Point;
use num_bigint_dig::BigInt;
use num_traits::Zero;

// Performs elliptic curve point multiplication to compute the public key.
//
// # Arguments
// * `scalar` - The private key scalar to multiply the base point by.
// * `point` - The base point on the elliptic curve.
// * `a` - The 'a' coefficient of the elliptic curve.
// * `_b` - The 'b' coefficient of the elliptic curve (unused but kept for interface consistency).
// * `modulus` - The modulus defining the finite field.
//
// # Returns
// A `Result` containing the resulting point (the public key) or an error.
pub fn elliptic_curve_multiply(
    scalar: &BigInt,
    point: &Point,
    a: &BigInt,
    _b: &BigInt,
    modulus: &BigInt,
) -> Result<Point, Box<dyn std::error::Error>> {
    let mut result = Point::Infinity;
    let mut temp = point.clone();
    let scalar_bits = scalar.to_bytes_be().1; // Get binary representation of scalar

    for bit in scalar_bits.iter().rev() {
        for i in (0..8).rev() {
            if (bit >> i) & 1 == 1 {
                // Point addition
                result = match (result.clone(), temp.clone()) {
                    (Point::Infinity, p) | (p, Point::Infinity) => p,
                    (Point::Coordinates(x1, y1), Point::Coordinates(x2, y2)) => {
                        if x1 == x2 && y1 == -&y2 {
                            Point::Infinity
                        } else {
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

                            let x3 = (&slope * &slope - &x1 - &x2) % modulus;
                            let y3 = (&slope * (&x1 - &x3) - &y1) % modulus;
                            Point::Coordinates(x3, y3)
                        }
                    }
                };
            }
            // Point doubling for each bit
            temp = match temp {
                Point::Infinity => Point::Infinity,
                Point::Coordinates(x, y) => {
                    if y.is_zero() {
                        Point::Infinity
                    } else {
                        let slope = {
                            let numerator = BigInt::from(3) * &x * &x + a;
                            let denominator = BigInt::from(2) * &y;
                            let inv = crate::mod_inverse(&denominator, modulus)?;
                            (numerator * inv) % modulus
                        };
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
