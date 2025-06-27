// src/hokg.rs

use crate::{point::Point, Config, HokgResult};
use num_bigint_dig::BigInt;
use num_traits::{Pow, ToPrimitive};
use rand::rngs::OsRng;

// Generates a key pair using the HOKG algorithm with Hensel lifting for elliptic curves.
//
// # Arguments
// * `config` - Configuration parameters for the elliptic curve and Hensel lifting process.
//
// # Returns
// A `HokgResult` containing:
// - The base point on the elliptic curve,
// - The generated private key,
// - The computed public key,
// - Minimal configuration data for potential reuse.
pub fn hokg(config: Config) -> HokgResult {
    // Extract configuration values
    let p = BigInt::from(config.p);
    let a = BigInt::from(config.a);
    let b = BigInt::from(config.b);
    let x0 = BigInt::from(config.x0);
    let y0 = BigInt::from(config.y0);
    let k = config.k;

    // Compute the modulus for the elliptic curve operations
    let modulus = p.pow(k as u32);
    let modulus_u64 = modulus.to_u64().ok_or("Modulus too large for u64")?;

    // Perform Hensel lifting to obtain the base point coordinates at the k-th level
    let (x_k, y_k) = crate::utils::hensel_lift(&p, &a, &b, &x0, &y0, k)?;
    let base_point = Point::Coordinates(x_k.clone(), y_k.clone());

    // Initialize a cryptographically secure random number generator
    let mut rng = OsRng;

    // Generate a private key within the range [1, modulus_u64 - 1]
    let private_key = BigInt::from(
        rng.try_next_u64().ok_or("Failed to generate random u64")? % (modulus_u64 - 1) + 1,
    );

    // Compute the public key by multiplying the base point by the private key on the elliptic curve
    let public_key =
        crate::ecc::elliptic_curve_multiply(&private_key, &base_point, &a, &b, &modulus)?;

    // Prepare minimal data for return, consisting of the original configuration values
    let minimal_data = (config.p, config.a, config.b, config.x0, config.y0, config.k);

    // Return the result as a tuple wrapped in Ok
    Ok((base_point, private_key, public_key, minimal_data))
}
