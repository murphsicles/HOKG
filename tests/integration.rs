use hokg::{hokg, Point, Config};

#[test]
fn test_hokg_key_generation() {
    let config = Config {
        p: 5,  // Small prime
        a: 1,  // Curve parameter a
        b: 1,  // Curve parameter b
        x0: 2, // Seed x-coordinate
        y0: 3, // Seed y-coordinate
        k: 2,  // Lifting exponent
    };

    let result = hokg(config);

    assert!(result.is_ok());
    if let Ok((base_point, private_key, public_key, minimal_data)) = result {
        match base_point {
            Point::Coordinates(_, _) => assert!(true),
            Point::Infinity => assert!(false, "Base point should not be infinity"),
        }

        match public_key {
            Point::Coordinates(_, _) => assert!(true),
            Point::Infinity => assert!(false, "Public key should not be infinity"),
        }

        assert_eq!(minimal_data.0, 5);
        assert_eq!(minimal_data.1, 1);
        assert_eq!(minimal_data.2, 1);
        assert_eq!(minimal_data.3, 2);
        assert_eq!(minimal_data.4, 3);
        assert_eq!(minimal_data.5, 2);
    }
}
