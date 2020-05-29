/* links the standard library. Let's try to disable */
#![no_std]
/* Tell the Rust compiler that we don't want to use 
 * the normal entry point chain
 */
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Fuck yeah!";

/* The panic_handler attribute defines the function that the compiler
 * should invoke when a panic occurs. The standard library provides
 * its own panic handler function, but in a no_std environment we
 * need to define it ourselves
 */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/* 
 * the #[no_mangle] attribute we disable the name mangling to ensure
 * that the Rust compiler really outputs a function with the name
 * _start. Without the attribute, the compiler would generate some
 * cryptic _ZN3blog_os4_start7hb173fedf945531caE 
 *
 * extern "C" to tell the compiler that it should use the C 
 * calling convention for this function 
 * 
 * The ! return type means that the function is diverging, i.e. 
 * not allowed to ever return
 */
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    
    loop {}
}
