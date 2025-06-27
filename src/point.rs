// src/point.rs

use num_bigint_dig::BigInt;

// Represents a point on an elliptic curve (x, y) or the point at infinity.
#[derive(Clone, Debug, PartialEq)]
pub enum-Point {
    Infinity,
    Coordinates(BigInt, BigInt),
}
