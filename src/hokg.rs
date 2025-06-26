// src/hokg.rs

use num_bigint::BigInt;
use num_traits::ToPrimitive;
use rand::Rng;
use std::error::Error;

use crate::ecc::{elliptic_curve_multiply, Point};
use crate::hensel::hensel_lift;

/// Configuration for the HOKG algorithm.
#[derive(Debug, Clone)]
pub struct Config {
    pub p: u64,   // Small prime
    pub a: i64,   // Curve parameter a
    pub b: i64,   // Curve parameter b
    pub x0: i64,  // Seed x-coordinate
    pub y0: i64,  // Seed y-coordinate
    pub k: usize, // Lifting exponent
}

/// Return type alias for the HOKG algorithm result.
type HokgResult = Result<(Point, BigInt, Point, (u64, i64, i64, i64, i64, usize)), Box<dyn Error>>;

/// Runs the HOKG algorithm to generate an ECC key pair.
pub fn hokg(config: Config) -> HokgResult {
    let p = BigInt::from(config.p);
    let a = BigInt::from(config.a);
    let b = BigInt::from(config.b);
    let x0 = BigInt::from(config.x0);
    let y0 = BigInt::from(config.y0);
    let k = config.k;

    // Step 1: Hensel lifting to get base point
    let (x_k, y_k) = hensel_lift(&p, &a, &b, &x0, &y0, k)?;
    let base_point = Point::Coordinates(x_k.clone(), y_k.clone());
    let modulus = p.pow(k as u32);

    // Step 2: Generate private key (simplified range for demo)
    let modulus_u64 = modulus.to_u64().ok_or("Modulus too large for u64")?;
    let private_key = BigInt::from(rand::rng().random_range(1..modulus_u64));

    // Step 3: Compute public key
    let public_key = elliptic_curve_multiply(&private_key, &base_point, &a, &b, &modulus)?;

    // Step 4: Minimal data for transmission
    let minimal_data = (config.p, config.a, config.b, config.x0, config.y0, config.k);

    Ok((base_point, private_key, public_key, minimal_data))
}
