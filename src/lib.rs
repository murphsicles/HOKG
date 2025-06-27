// src/lib.rs

use num_bigint_dig::BigInt;
use std::error::Error;

// Declare the modules for ecc, hensel, hokg, point, and utils
pub mod ecc;
pub mod hensel;
pub mod hokg;
pub mod point;
pub mod utils;

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

// Re-export Config from the hokg module
pub use hokg::Config;

// Re-export Point from the point module
pub use point::Point;

// Export the hokg function from the hokg module
pub use hokg::hokg;

// Export the elliptic_curve_multiply function from the ecc module
pub use ecc::elliptic_curve_multiply;

// Export the mod_inverse function from the utils module
pub use utils::mod_inverse;
