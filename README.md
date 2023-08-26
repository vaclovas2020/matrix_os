# Matrix OS Kernel written in Rust language

Install nightly rust:
``
 rustup toolchain install nightly
``

Add rust-src component:
``
rustup component add rust-src --toolchain nightly
``

Install llvm-tools-preview:
``
rustup component add llvm-tools-preview --toolchain nightly
``

Install bootimage:
``
cargo install bootimage
``

Build kernel with command (for x86_64 target):
``
cargo +nightly build
``

Create boot image with command:
``
cargo +nightly bootimage
``
Run and test OS kernel on QEMU ([Install qemu first](https://www.qemu.org/download)):
``
cargo +nightly run
``

Run tests with:
``
cargo +nightly test
``