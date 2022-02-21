#![no_std]
#![no_main]

use arduino_hal::{prelude::*, delay_ms};
use panic_halt as _;
use embedded_hal::serial::Read;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut led_s1 = pins.d2.into_output().downgrade();
    let mut led_s2 = pins.d3.into_output().downgrade();
    let mut led_s4 = pins.d4.into_output().downgrade();
    let mut led_s8 = pins.d5.into_output().downgrade();
    let mut led_b1 = pins.d6.into_output().downgrade();
    let mut led_b2 = pins.d7.into_output().downgrade();
    let mut led_b4 = pins.d8.into_output().downgrade();
    let mut led_b8 = pins.d9.into_output().downgrade();
    let mut led_b16 = pins.d10.into_output().downgrade();
    let mut led_b32 = pins.d11.into_output().downgrade();
    let mut led_b64 = pins.d12.into_output().downgrade();

    let mut small_leds = [
        led_s8,
        led_s4,
        led_s2,
        led_s1,
    ];
    let mut big_leds = [
        led_b64,
        led_b32,
        led_b16,
        led_b8,
        led_b4,
        led_b2,
        led_b1,
    ];
    let mut delay = 100;

    loop {
        // Read a byte from the serial connection
        //let b = nb::block!(serial.read()).void_unwrap();

        // Answer
        //ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();
        let mut leds_b = big_bits_from_integer(35);
        let mut leds_s = small_bits_from_integer(9);
        for i in 0..6 {
            if leds_b[i] {
                big_leds[i].set_high();
            } else {
                big_leds[i].set_low();
            }
        }
        for i in 0..3 {
            if leds_s[i] {
                small_leds[i].set_high();
            } else {
                small_leds[i].set_low();
            }
        }
        delay_ms(delay);
    }
}

fn big_bits_from_integer(integer: u8) -> [bool; 7] {
    let mut bits = [false; 7];
    for i in 0..6 {
        bits[i] = (integer & (1 << i)) != 0;
    }
    bits
}

fn small_bits_from_integer(integer: u8) -> [bool; 4] {
    let mut bits = [false; 4];
    for i in 0..3 {
        bits[i] = (integer & (1 << i)) != 0;
    }
    bits
}