# spiderbot-rust
Hexapod robot software in Rust

Device Setup
-------------

1. Install `rustup`, `avr-gcc`, `avr-libc`, `avrdude`, `systemd-devel`, and `pkgconf-pkg-config` from the system repo.
2. Switch to nightly for the project folder: `rustup override set nightly`
3. Download and extract the AVR toolchain from the [manufacturer's website](https://www.microchip.com/en-us/tools-resources/develop/microchip-studio/gcc-compilers).
4. Add the path to the toolchain's `bin` folder to the system path.
5. Install ravedude: `cargo install --locked ravedude`
