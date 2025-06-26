// src/lib.rs

/// Hensel-Optimized Key Generation (HOKG) library for data-efficient ECC key generation.
///
/// This library implements the HOKG algorithm, leveraging Hensel's lemma to lift
/// low-precision points to high-precision ECC base points, reducing data usage
/// by up to 40%. Suitable for resource-constrained environments like IoT.
///
/// # Example
/// ```rust
/// use hokg::{hokg, Config};
/// let config = Config {
///     p: 17,
///     a: 2,
///     b: 3,
///     x0: 5,
///     y0: 6,
///     k: 5,
/// };
/// let (base_point, private_key, public_key, minimal_data) = hokg(config).unwrap();
/// ```
pub mod ecc;
pub mod hensel;
pub mod hokg;
pub mod utils;
pub use ecc::Point;
pub use hokg::{hokg, Config};
