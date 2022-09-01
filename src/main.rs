#![no_std]
#![no_main]

use atmega_hal::{usart::BaudrateExt, Peripherals, Usart};
use avr_hal_generic::prelude::*;
use core::panic::PanicInfo;
use ufmt::uWrite;

type DefaultClock = atmega_hal::clock::MHz16;

#[avr_device::entry]
unsafe fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);

    let mut led = pins.pa4.into_output();

    let pd0 = pins.pd0.into_pull_up_input();
    let pd1 = pins.pd1.into_output();

    let mut serial = atmega_hal::Usart::<_, _, _, DefaultClock>::new(
        dp.USART0,
        pd0,
        pd1,
        57_600.into_baudrate(),
    );

    let mut delay = atmega_hal::delay::Delay::<DefaultClock>::new();

    loop {
        led.set_high();

        serial.write_str("hellOwOrld (◡ ω ◡)\n").void_unwrap();
        serial.flush();

        led.set_low();
        delay.delay_ms(500u16);
    }
}

#[panic_handler]
fn panic_handler(p: &PanicInfo) -> ! {
    avr_device::interrupt::disable();

    let dp = unsafe { Peripherals::steal() };
    let pins = atmega_hal::pins!(dp);

    let mut serial = Usart::<_, _, _, DefaultClock>::new(
        dp.USART0,
        pins.pd0,
        pins.pd1.into_output(),
        57_600.into_baudrate(),
    );

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
