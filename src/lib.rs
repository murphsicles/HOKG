// src/lib.rs

use num_bigint_dig::BigInt;
use std::error::Error;

// Declare the modules for ecc, hensel, hokg, point, and utils
pub mod ecc;
pub mod hensel;
pub mod hokg;
pub mod point;
pub mod utils;

// Define the Config struct publicly
// This struct holds the configuration parameters for the elliptic curve and Hensel lifting
pub struct Config {
    pub p: u64,   // Small prime
    pub a: i64,   // Curve parameter a
    pub b: i64,   // Curve parameter b
    pub x0: i64,  // Seed x-coordinate
    pub y0: i64,  // Seed y-coordinate
    pub k: usize, // Lifting exponent
}

// Define the HokgResult type publicly
// This type alias represents the result of the HOKG algorithm, containing the base point,
// private key, public key, and minimal configuration data
pub type HokgResult = Result<
    (
        point::Point,                     // Base point
        BigInt,                           // Private key
        point::Point,                     // Public key
        (u64, i64, i64, i64, i64, usize), // Minimal configuration data
    ),
    Box<dyn Error>, // Error type
>;

// Re-export Point from the point module
pub use point::Point;

// Export the hokg function from the hokg module
pub use hokg::hokg;

// Export the elliptic_curve_multiply function from the ecc module
pub use ecc::elliptic_curve_multiply;

// Export the mod_inverse function from the utils module
pub use utils::mod_inverse;
