# Raspberry Pi Build Hat Serial
### A rust library made to control the build hat using the rust programming language.
#### Made by the RSHS robotics club
## Requirements
- Git
- Cargo
- VS Code is recommended
---
## Installation
1. to make a new rust project, go to the directory that you wanted to store it and run
```cargo new <project name>```.
it should automaticly make a file with everything you need.
> you can open a powershell window by pressing shift + rightclick in the file explorer


2. Go to your ```Cargo.toml``` file, and type in ```rpi_build_hat_serial = { git = "https://github.com/rshs-robotics-club/rpi_build_hat_serial.git", rev = "<version>" }```under `[dependencies]`.

3. Build the file. Type the following into your terminal:
    1. 32-bit version:`cross build --target arm-unknown-linux-musleabihf`
    2. 64-bit version:`cross build --target aarch64-unknown-linux-gnu`
---
If you have any questions, feel free to ask in the discussions page.
