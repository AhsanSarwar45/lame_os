#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;


// PanicInfo contains the file and line no. where the panic occurs
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
  
    loop {}
}
