use num_bigint_dig::BigInt;
use std::error::Error;

// Declare modules publicly
pub mod hokg;
pub mod point;

// Define Config publicly
pub struct Config {
    pub p: u64,   // Small prime
    pub a: i64,   // Curve parameter a
    pub b: i64,   // Curve parameter b
    pub x0: i64,  // Seed x-coordinate
    pub y0: i64,  // Seed y-coordinate
    pub k: usize, // Lifting exponent
}

// Define HokgResult publicly
pub type HokgResult = Result<
    (
        point::Point,                     // Base point
        BigInt,                           // Private key
        point::Point,                     // Public key
        (u64, i64, i64, i64, i64, usize), // Minimal configuration data
    ),
    Box<dyn Error>,
>;

// Re-export useful items
pub use hokg::hokg;
pub use point::Point;
