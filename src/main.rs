#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut red_led = pins.d8.into_output();
    let mut yellow_led = pins.d9.into_output();
    let mut green_led = pins.d10.into_output();

    loop {
        red_led.set_high();
        yellow_led.set_low();
        green_led.set_low();
        delay_ms(1500);

        red_led.set_high();
        yellow_led.set_high();
        green_led.set_low();
        delay_ms(500);

        red_led.set_low();
        yellow_led.set_low();
        green_led.set_high();
        delay_ms(1500);

        red_led.set_low();
        yellow_led.set_high();
        green_led.set_low();
        delay_ms(1500);
    }
}
