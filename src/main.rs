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

    let mut motor_enable = pins.xy_enable.into_output();
    let mut motor_dir = pins.x_dir.into_output();
    let mut motor_step = pins.x_step.into_output();

    const MOTOR_DELAY_DIR_MIN_NS: u32 = 200;
    const MOTOR_DELAY_STEP_MIN_US: u32 = 1;
    const MOTOR_MAX_STEPPING_RATE_HZ: u32 = 500000;

    led.set_high();
    arduino_hal::delay_ms(100);
    led.set_low();

    for _ in 0..200*16 {
        motor_step.set_high();
        arduino_hal::delay_us(MOTOR_DELAY_STEP_MIN_US + 1);
        motor_step.set_low();
        arduino_hal::delay_us(MOTOR_DELAY_STEP_MIN_US + 1);
    }

    led.set_high();
    loop {}

    // loop {
    //     led.set_high();
    //
    //     serial.write_str("hellOwOrld (◡ ω ◡)\n").void_unwrap();
    //     serial.flush();
    //
    //     led.set_low();
    //     arduino_hal::delay_ms(500);
    // }
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
