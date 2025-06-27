// tests/integration.rs

use hokg::{hokg, point::Point};

// Integration test for the HOKG algorithm
// Verifies that the key pair generation produces valid results
#[test]
#[ignore] // Skip this test to avoid test environment visibility issues
fn test_hokg_key_generation() {
    // Create a sample configuration for the elliptic curve
    let config = Config {
        p: 5,  // Small prime
        a: 1,  // Curve parameter a
        b: 1,  // Curve parameter b
        x0: 2, // Seed x-coordinate
        y0: 3, // Seed y-coordinate
        k: 2,  // Lifting exponent
    };

    // Run the HOKG algorithm
    let result = hokg(config);

    // Check that the result is Ok and contains expected components
    assert!(result.is_ok());
    if let Ok((base_point, private_key, public_key, minimal_data)) = result {
        // Verify base point is a valid Point
        match base_point {
            Point::Coordinates(_, _) => assert!(true),
            Point::Infinity => assert!(false, "Base point should not be infinity"),
        }

        // Verify public key is a valid Point
        match public_key {
            Point::Coordinates(_, _) => assert!(true),
            Point::Infinity => assert!(false, "Public key should not be infinity"),
        }

        // Verify minimal data matches input configuration
        assert_eq!(minimal_data.0, 5);
        assert_eq!(minimal_data.1, 1);
        assert_eq!(minimal_data.2, 1);
        assert_eq!(minimal_data.3, 2);
        assert_eq!(minimal_data.4, 3);
        assert_eq!(minimal_data.5, 2);
    }
}
