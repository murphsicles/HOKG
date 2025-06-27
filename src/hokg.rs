// src/hokg.rs

use crate::{Config, HokgResult, point::Point};
use num_bigint_dig::BigInt;
use num_traits::Pow;
use rand::{thread_rng, Rng};

/// Runs the HOKG algorithm to generate an ECC key pair.
pub fn hokg(config: Config) -> HokgResult {
    let p = BigInt::from(config.p);
    let a = BigInt::from(config.a);
    let b = BigInt::from(config.b);
    let x0 = BigInt::from(config.x0);
    let y0 = BigInt::from(config.y0);
    let k = config.k;

    // Step 1: Hensel lifting to get base point
    let (x_k, y_k) = crate::hensel::hensel_lift(&p, &a, &b, &x0, &y0, k)?;
    let base_point = Point::Coordinates(x_k.clone(), y_k.clone());
    let modulus = p.pow(k as u32);

    // Step 2: Generate private key (simplified range for demo)
    let modulus_u64 = modulus.to_u64().ok_or("Modulus too large for u64")?;
    let private_key = BigInt::from(thread_rng().gen_range(1..modulus_u64));

    // Step 3: Compute public key
    let public_key = crate::ecc::elliptic_curve_multiply(&private_key, &base_point, &a, &b, &modulus)?;

    // Step 4: Minimal data for transmission
    let minimal_data = (config.p, config.a, config.b, config.x0, config.y0, config.k);

    Ok((base_point, private_key, public_key, minimal_data))
}
