#![no_std]
#![no_main]

use core::panic::PanicInfo;

// PanicInfo contains the file and line no. where the panic occurs
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
