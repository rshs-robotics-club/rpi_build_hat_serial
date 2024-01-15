# Raspberry Pi Build Hat Serial
### A rust library made to control the build hat using the rust programming language.

## Requirements
- Git
- Cargo
- VS Code is recommended
  
## Installation
1. to make a new rust project, go to the directory that you wanted to store it and run
```cargo new <project name>```.
it should automaticly make a file with everything you need.
> you can open a powershell window by pressing shift + rightclick in the file explorer


3. go to your ```Cargo.toml``` file, and type in ```rpi_build_hat_serial = { git = "https://github.com/rshs-robotics-club/rpi_build_hat_serial.git", rev = "<version>" }```under `[dependencies]`.
   ![image](https://github.com/rshs-robotics-club/rpi_build_hat_serial/assets/95858994/504a641b-d623-4f61-996c-821ff96e28a0)

4. build the file by typing `cross build --target=armv7-unknown-linux-gnueabihf` into the terminal.
![image](https://github.com/rshs-robotics-club/rpi_build_hat_serial/assets/95858994/fde53e42-64d6-4638-8523-826b1ff8794f)
> Note that different version of raspberry pi OS has different targets. This example uses the 32bit version.
