// examples/simple.rs

use hokg::{hokg, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        p: 17,
        a: 2,
        b: 3,
        x0: 5,
        y0: 6,
        k: 5,
    };
    let (base_point, private_key, public_key, minimal_data) = hokg(config)?;
    println!("Base Point: {:?}", base_point);
    println!("Private Key: {}", private_key);
    println!("Public Key: {:?}", public_key);
    println!("Minimal Data: {:?}", minimal_data);
    Ok(())
}
