# Raspberry Pi Build Hat Serial

A library that aims to provide rust support for the Raspberry Pi's Build Hat.

#### Made by the RSHS robotics club

## Requirements

- Git
- Cargo
- Cross
---
## How to use this library

1. Go to your ```Cargo.toml``` file and put this under `[dependencies]`:

```rpi_build_hat_serial = { git = "https://github.com/rshs-robotics-club/rpi_build_hat_serial.git", rev = "<version>" }```

## Building

1. 32-bit version:`cross build --target arm-unknown-linux-musleabihf`
2. 64-bit version:`cross build --target aarch64-unknown-linux-gnu`
3. 
---

Questions and Suggestions
--------------------

you can ask questions and give suggestions at the [Discussions Page](https://github.com/rshs-robotics-club/rpi_build_hat_serial/discussions)

---

Remember to check the [Rust Book](https://doc.rust-lang.org/book/) before you use this library!

