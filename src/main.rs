#![no_std]
#![no_main]

use arduino_hal::{prelude::*, delay_ms};
use panic_halt as _;
use embedded_hal::serial::Read;
use embedded_hal::digital::v2::OutputPin;
//use dht11::Dht11;
use dht_sensor::*;

#[arduino_hal::entry]
fn main() -> ! {
	let dp = arduino_hal::Peripherals::take().unwrap();
	let pins = arduino_hal::pins!(dp);
	let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

	let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

	// Create an instance of the DHT11 device

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

	//let a1 = pins.a1;
	//let a1 = pins.a1.into_pull_up_input().downgrade();
	//let a1 = pins.a1.into_output();
	let a1 = pins.a1.into_analog_input(&mut adc);
	//let a1 = pins.a1.into_floating_input().downgrade();
	//let d13 = pins.d13.into_pull_up_input();

	//let mut dht11 = Dht11::new(d13);

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
	//let mut delay = 100;
	//let mut delay = delay::Delay::new(cp.SYST, &rcc);

	loop {
		let values = [
			a1.analog_read(&mut adc),
		];
		/*
		match dht11::Reading::read(&mut delay, &mut a1) {
            Ok(dht11::Reading {
                temperature,
                relative_humidity,
            }) => ufmt::uwrite!(&mut serial, "{}°, {}% RH", temperature, relative_humidity).void_unwrap(),
            Err(e) => ufmt::uwrite!(&mut serial, "lol").void_unwrap(),
        }*/
		//let measurement = dht11.perform_measurement(&mut delay).unwrap();
		/*
		for (i, v) in values.iter().enumerate() {
			ufmt::uwrite!(&mut serial, "A{}: {} ", i, v).void_unwrap();
		}*/
		// Read a byte from the serial connection
		//let b = nb::block!(serial.read()).void_unwrap();

		// Answer
		//ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();
		let mut leds_b = big_bits_from_integer(2);
		let mut leds_s = small_bits_from_integer(2);
		for j in 0..4 {
			if leds_s[j] {
				small_leds[j].set_high();
			} else {
				small_leds[j].set_low();
			}
		}
		for i in 0..7 {
			if leds_b[i] {
				big_leds[i].set_high();
			} else {
				big_leds[i].set_low();
			}
		}
		ufmt::uwrite!(&mut serial, "A1: {} ", values[0]).void_unwrap();
		//ufmt::uwrite!(&mut serial, "A1: {} ", measurement).void_unwrap();
		delay_ms(1000);
	}
}

fn big_bits_from_integer(integer: u8) -> [bool; 8] {
	let mut bits = [true; 8];
	for i in 0..8{
		bits[i] = (integer & (1 << i)) != 0;
	}
	bits
}

fn small_bits_from_integer(integer: u8) -> [bool; 4] {
	let mut bits = [false; 4];
	for i in 0..4 {
		bits[i] = (integer & (1 << i)) != 0;
		//bits[i] = true;
	}
	bits
}