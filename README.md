# Embedded Rust with Embassy

Base Rust project for BlackPill with STM32F411CEU6 and embassy.

Video that explains most of the setup [Embedded Rust setup explained](https://www.youtube.com/watch?v=TOAynddiu5M)

## Windows build tools
[Visual Studio C++ Build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- MSVC v143 - VS 2022 C++ x64/x86 build tools (latest)
- Windows 11 SDK
- C++ CMake tools for Windows (kjøres fra developer prompt)

## Rust setup
[Install Rust](https://www.rust-lang.org/tools/install)

## Flash and debug
[probe-rs](https://www.rust-lang.org/tools/install)

Usefull commands:
```powershell
cargo install probe-rs-tools --locked --force
probe-rs chip list | select-string stm32f411ce

rustup update
rustup target add thumbv7em-none-eabihf
rustup show

rustup component add llvm-tools
cargo install cargo-binutils
```

If trouble flashing target:
```
 WARN probe_rs::probe::stlink: send_jtag_command 242 failed: SwdDpWait
 WARN probe_rs::probe::stlink: got SwdDpWait/SwdApWait, retrying.
 WARN probe_rs::probe::stlink: send_jtag_command 242 failed: SwdDpWait
 WARN probe_rs::probe::stlink: got SwdDpWait/SwdApWait, retrying.
```
- Press and hold **BOOT0**
- Press and releas **RESET**
- Release **BOOT0**

## VS Code

Extensions:
- rust-analyzer
- Even Better TOML
- Error Lens
- Debugger for probe-rs

