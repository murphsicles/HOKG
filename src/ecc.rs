// src/ecc.rs

use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::error::Error;

use crate::utils::{gcd, mod_inverse};

/// Represents a point on an elliptic curve (x, y) or the point at infinity (None).
#[derive(Debug, Clone, PartialEq)]
pub enum Point {
    Infinity,
    Coordinates(BigInt, BigInt),
}

/// Performs scalar multiplication `d * point` on the elliptic curve y^2 = x^3 + ax + b mod modulus.
#[allow(unused_variables)]
pub fn elliptic_curve_multiply(
    d: &BigInt,
    point: &Point,
    a: &BigInt,
    b: &BigInt,
    modulus: &BigInt,
) -> Result<Point, Box<dyn Error>> {
    if d.is_zero() {
        return Ok(Point::Infinity);
    }

    let mut result = Point::Infinity;
    let mut current = point.clone();
    let d_bits = format!("{:b}", d); // Binary representation of d

    for bit in d_bits.chars() {
        // Double the result point
        result = match &result {
            Point::Infinity => Point::Infinity,
            Point::Coordinates(x, y) => {
                if y.is_zero() {
                    Point::Infinity
                } else {
                    // Slope: s = (3x^2 + a) / (2y)
                    let three = BigInt::from(3);
                    let two = BigInt::from(2);
                    let s_num = (three * x * x + a) % modulus;
                    let s_den = (two.clone() * y) % modulus;
                    if gcd(&s_den, modulus) != BigInt::one() {
                        return Err("Doubling failed: denominator not invertible".into());
                    }
                    let s = (s_num * mod_inverse(&s_den, modulus)?) % modulus;
                    // x' = s^2 - 2x
                    let x_new = (s.clone() * s.clone() - x * two) % modulus;
                    // y' = s(x - x') - y
                    let y_new = (s * (x - &x_new) - y) % modulus;
                    Point::Coordinates(x_new, y_new)
                }
            }
        };

        if bit == '1' {
            // Add current to result
            result = match (&result, &current) {
                (Point::Infinity, _) => current.clone(),
                (_, Point::Infinity) => result.clone(),
                (Point::Coordinates(x1, y1), Point::Coordinates(x2, y2)) => {
                    if x1 == x2 && y1 == &(-y2 % modulus) {
                        Point::Infinity
                    } else if x1 == x2 {
                        result.clone() // Same point, already doubled
                    } else {
                        // Slope: s = (y2 - y1) / (x2 - x1)
                        let s_num = (y2 - y1) % modulus;
                        let s_den = (x2 - x1) % modulus;
                        if gcd(&s_den, modulus) != BigInt::one() {
                            return Err("Addition failed: denominator not invertible".into());
                        }
                        let s = (s_num * mod_inverse(&s_den, modulus)?) % modulus;
                        // x' = s^2 - x1 - x2
                        let x_new = (s.clone() * s.clone() - x1 - x2) % modulus;
                        // y' = s(x1 - x') - y1
                        let y_new = (s * (x1 - &x_new) - y1) % modulus;
                        Point::Coordinates(x_new, y_new)
                    }
                }
            };
        }

        // Double the current point for the next iteration
        current = match &current {
            Point::Infinity => Point::Infinity,
            Point::Coordinates(x, y) => {
                if y.is_zero() {
                    Point::Infinity
                } else {
                    let three = BigInt::from(3);
                    let two = BigInt::from(2);
                    let s_num = (three * x * x + a) % modulus;
                    let s_den = (two.clone() * y) % modulus;
                    if gcd(&s_den, modulus) != BigInt::one() {
                        return Err("Current doubling failed: denominator not invertible".into());
                    }
                    let s = (s_num * mod_inverse(&s_den, modulus)?) % modulus;
                    // x' = s^2 - 2x
                    let x_new = (s.clone() * s.clone() - x * two) % modulus;
                    // y' = s(x - x') - y
                    let y_new = (s * (x - &x_new) - y) % modulus;
                    Point::Coordinates(x_new, y_new)
                }
            }
        };
    }

    Ok(result)
}
