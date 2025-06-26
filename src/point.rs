// src/point.rs

use num_bigint::BigInt;

/// Represents a point on an elliptic curve (x, y) or the point at infinity.
#[derive(Debug, Clone, PartialEq)]
pub enum Point {
    Infinity,
    Coordinates(BigInt, BigInt),
}
