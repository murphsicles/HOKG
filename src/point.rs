// src/point.rs

use num_bigint_dig::BigInt;

// Represents a point on an elliptic curve, either as coordinates (x, y) or the point at infinity
#[derive(Clone, Debug, PartialEq)]
pub enum Point {
    Coordinates(BigInt, BigInt), // Point with x and y coordinates
    Infinity,                    // Point at infinity
}
