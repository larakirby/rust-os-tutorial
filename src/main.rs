#![no_std]
#![no_main]

use core::panic::PanicInfo;

// do something if there's an error.
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    // _info has file/line of panic and msg if there
    loop {}
}

// language items: stack unwinding - os specific libs...TODO?


// entry point
// no mangle says don't come up with a unique name for this
// extern "C" means use C calling convention
// _start is a common entry point name
// ! return means this function isn't allowed to return - not called, but invoked by OS
#[no_mangle]
pub extern "C" fn _start()->!{
    loop{}
}

// add a target:
// run rustup target add thumbv7em-none-eabihf
// then cargo build --target thumbv7em-none-eabihf 



