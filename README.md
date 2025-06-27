# Hensel-Optimized Key Generation (HOKG) 🚀

[![CI](https://github.com/murphsicles/HOKG/actions/workflows/ci.yml/badge.svg)](https://github.com/murphsicles/HOKG/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/hokg.svg)](https://crates.io/crates/hokg)
[![Dependencies](https://deps.rs/repo/github/murphsicles/HOKG/status.svg)](https://deps.rs/repo/github/murphsicles/HOKG)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.83+-orange.svg)](https://www.rust-lang.org/)

**HOKG** is a Rust library implementing the Hensel-Optimized Key Generation algorithm for elliptic curve cryptography (ECC). By leveraging Hensel's lemma, HOKG achieves up to 40% data efficiency in key generation, making it ideal for resource-constrained environments like IoT and edge devices. This library is efficient, secure, and built with modern Rust best practices.

📚 Inspired by the article by Dr. Roy Murphy: [Revolutionizing Elliptic Curve Cryptography](https://medium.com/@DrRoyMurphy/revolutionizing-elliptic-curve-cryptography-a-novel-application-of-hensels-lemma-for-d84d53c3a9ba)

## ✨ Features
- **Data Efficiency**: Reduces key generation data footprint by up to 40%.
- **Hensel's Lemma**: Novel application for lifting ECC parameters.
- **Secure**: Maintains ECC security based on the discrete logarithm problem.
- **Modular Design**: Clean separation of Hensel lifting, ECC, and utilities.
- **Comprehensive Tests**: Ensures reliability and correctness.

## 🛠️ Installation
Add HOKG to your project with:

```toml
[dependencies]
hokg = "0.1.0"
```

Ensure you have Rust 1.80 or later installed.

## 🚀 Quick Start
```rust
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
```

Run the example:
```bash
cargo run --example simple
```

## 📖 Documentation
Full API documentation is available via:
```bash
cargo doc --open
```

## 🧪 Testing
Run the test suite:
```bash
cargo test
```

Run benchmarks:
```bash
cargo bench
```

## 🤝 Contributing
Contributions are welcome! Please:
1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/awesome-feature`).
3. Commit your changes (`git commit -m 'Add awesome feature'`).
4. Push to the branch (`git push origin feature/awesome-feature`).
5. Open a Pull Request.

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## 📜 License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file.

## 🔒 Security
This is a cryptographic library. 
For production use, ensure:
- Use standardized curves (e.g., NIST P-256).
- Validate curve order and points.
- Consult a cryptography expert.

## 📬 Contact
For questions, open an issue or contact [Dr. Roy Murphy](https://github.com/murphsicles).

---

Built with ❤️ using Rust 🦀
