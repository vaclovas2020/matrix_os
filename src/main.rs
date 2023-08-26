#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(matrix_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use matrix_os::println;

const OS_VERSION_MAJOR: u8 = 0x00;
const OS_VERSION_MINOR: u8 = 0x01;
const OS_VERSION_PATCH: u8 = 0x00;

// our existing panic handler
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    matrix_os::test_panic_handler(info)
}

// This is kernel entry point function
#[no_mangle]
pub extern "C" fn _start() {
    println!(
        "Welcome to MATRIX OS {}.{}.{}\n",
        OS_VERSION_MAJOR, OS_VERSION_MINOR, OS_VERSION_PATCH
    );

    matrix_os::init();

    #[cfg(test)]
    test_main();

    loop {}
}
