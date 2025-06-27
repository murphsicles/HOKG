// src/hokg.rs

use crate::{Config, HokgResult};
use num_bigint_dig::BigInt;
use rand::{rng, Rng};

/// Generates a key pair using the HOKG algorithm.
///
/// # Arguments
/// * `config` - Configuration parameters for the elliptic curve and Hensel lifting.
///
/// # Returns
/// A `HokgResult` containing the base point, private key, public key, and minimal configuration data.
pub fn hokg(config: Config) -> HokgResult {
    // Extract values from config
    let p = BigInt::from(config.p);
    let a = BigInt::from(config.a);
    let b = BigInt::from(config.b);
    let x0 = BigInt::from(config.x0);
    let y0 = BigInt::from(config.y0);
    let k = config.k;

    // Compute modulus
    let modulus = p.pow(k as u32);
    let modulus_u64 = modulus.to_u64().ok_or("Modulus too large for u64")?;

    // Perform Hensel lifting to get base point coordinates
    let (x_k, y_k) = crate::hensel::hensel_lift(&p, &a, &b, &x0, &y0, k)?;
    let base_point = crate::point::Point::Coordinates(x_k.clone(), y_k.clone());

    // Generate private key using updated random functions
    let private_key = BigInt::from(rng().random_range(1..modulus_u64));

    // Compute public key using elliptic curve multiplication
    let public_key = 
        crate::ecc::elliptic_curve_multiply(&private_key, &base_point, &a, &b, &modulus)?;

    // Package minimal data for return
    let minimal_data = (config.p, config.a, config.b, config.x0, config.y0, config.k);

    // Return the result as a tuple wrapped in Ok
    Ok((base_point, private_key, public_key, minimal_data))
}
