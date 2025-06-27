// examples/simple.rs

use hokg::{hokg, Config};

// Simple example demonstrating the use of the HOKG algorithm
fn main() {
    // Create a sample configuration for the elliptic curve
    let config = Config {
        p: 5,      // Small prime
        a: 1,      // Curve parameter a
        b: 1,      // Curve parameter b
        x0: 2,     // Seed x-coordinate
        y0: 3,     // Seed y-coordinate
        k: 2,      // Lifting exponent
    };

    // Run the HOKG algorithm and print the result
    match hokg(config) {
        Ok((base_point, private_key, public_key, minimal_data)) => {
            println!("Base Point: {:?}", base_point);
            println!("Private Key: {}", private_key);
            println!("Public Key: {:?}", public_key);
            println!("Minimal Data: {:?}", minimal_data);
        }
        Err(e) => println!("Error: {}", e),
    }
}
