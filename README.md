# SeDebugAbuse-rs with Rust 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

This repository features code written in Rust intended to exploit the SeDebugPrivilege privilege. With this privilege enabled, it is possible to perform a process injection attack on a target process that has administrator permissions.

- [Compile](#compile)
- [Usage](#usage)

# Compile

First perform the compilation with the command:
```sh
cargo build --release
```
If you are using a different operating system, you can use rustup and add the windows architecture:
```sh
rustup target add x86_64-pc-windows-gnu
```
Then compile specifying the architecture:
```sh
cargo build --release --target x86_64-pc-windows-gnu
```

# Usage

These are the two ways the code:
```sh
cargo run -- <pid>
```
```sh
SeDebugAbuse_rs.exe <pid> 
```
