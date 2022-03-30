#![no_std]
#![no_main]

use core::panic::PanicInfo;

// PanicInfo contains the file and line no. where the panic occurs
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 0xb8000 is mapped to the VGA text buffer https://en.wikipedia.org/wiki/VGA_text_mode
    let vga_buffer : *mut u8 = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // |Character (8bits) |Foreground color (3 bits)|Brightness (1 bit) |Background color (3 bits)| Blink (1 bit)|
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xcb;
        }
    }
    

    loop {}
}
