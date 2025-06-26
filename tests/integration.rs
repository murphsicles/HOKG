// tests/integration.rs

use hokg::{hokg, Config, Point};
use num_traits::Zero;

// Fallback import for debugging, in case hokg::Point fails
use hokg::ecc::Point as EccPoint;

#[test]
fn test_hokg_valid_config() {
    let config = Config {
        p: 17,
        a: 2,
        b: 3,
        x0: 5,
        y0: 6,
        k: 5,
    };
    let result = hokg(config);
    assert!(result.is_ok(), "HOKG failed: {:?}", result.err());
    let (base_point, private_key, public_key, minimal_data) = result.unwrap();
    assert!(matches!(base_point, Point::Coordinates(_, _)));
    assert!(!private_key.is_zero());
    assert!(matches!(
        public_key,
        Point::Coordinates(_, _) | Point::Infinity
    ));
    assert_eq!(minimal_data, (17, 2, 3, 5, 6, 5));
}

#[test]
fn test_hokg_invalid_seed() {
    let config = Config {
        p: 17,
        a: 2,
        b: 3,
        x0: 5,
        y0: 7, // Invalid: does not satisfy curve equation
        k: 5,
    };
    let result = hokg(config);
    assert!(result.is_err(), "Expected error for invalid seed");
}
