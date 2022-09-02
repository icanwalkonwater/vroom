#![no_std]
#![no_main]

use arduino_hal::{prelude::*, Peripherals};
use core::panic::PanicInfo;
use ufmt::uWrite;

#[arduino_hal::entry]
unsafe fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.ext_a4.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 57_600);

    loop {
        led.set_high();

        serial.write_str("hellOwOrld (◡ ω ◡)\n").void_unwrap();
        serial.flush();

        led.set_low();
        arduino_hal::delay_ms(500);
    }
}

#[panic_handler]
fn panic_handler(p: &PanicInfo) -> ! {
    avr_device::interrupt::disable();

    let dp = unsafe { Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57_600);

    serial.write_str("Panicked").void_unwrap();
    if let Some(location) = p.location() {
        ufmt::uwriteln!(
            &mut serial,
            " at {}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        )
        .void_unwrap();
    }

    loop {}
}
