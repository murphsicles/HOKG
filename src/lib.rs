// src/lib.rs

use num_bigint_dig::BigInt;
use std::error::Error;

// Declare the modules
pub mod point;
pub mod hokg;
pub mod ecc;

// Define Config struct publicly
pub struct Config {
    pub p: u64,   // Small prime
    pub a: i64,   // Curve parameter a
    pub b: i64,   // Curve parameter b
    pub x0: i64,  // Seed x-coordinate
    pub y0: i64,  // Seed y-coordinate
    pub k: usize, // Lifting exponent
}

// Define HokgResult type publicly
pub type HokgResult =
    Result<(point::Point, BigInt, point::Point, (u64, i64, i64, i64, i64, usize)), Box<dyn Error>>;

// Re-export Point from the point module
pub use point::Point;

// Export the hokg function from the hokg module
pub use hokg::hokg;

// Export other necessary items
pub use ecc::elliptic_curve_multiply;
