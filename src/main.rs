#![no_std]
// Main function is usually called by a runtime. In Rust, crt0 creates a stack and places arguments
// to the right registers, and invokes entrypoint of the Rust runtime (start), which only takes
// care of printing backtrace on panic and stack overflow guards. This runtime calls main function.
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
// The custom test harness generates a "main" function which calls test runner, but we use no_mean
// feature and so is ignored. We rename it to test_main and run from our own entrypoint for now.
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rust_os::println;

// our existing panic handler
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

// Linker expects this name for entrypoint.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rust_os::init();

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    #[cfg(test)]
    test_main();

    loop {}
}
