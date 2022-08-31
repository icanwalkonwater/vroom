#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[avr_device::entry]
fn main() -> ! {
    loop {}
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
