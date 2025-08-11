#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused_imports)]

use panic_halt as _;

use nb::block;

// use alloc::{boxed::Box, string::String, vec::Vec};
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use stm32f1xx_hal::{gpio::PinState, pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpio = dp.GPIOB.split();

    let mut led = gpio
        .pb0
        .into_open_drain_output_with_state(&mut gpio.crl, PinState::High);

    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(4.Hz()).unwrap();

    let mut _cnt: u32 = 0;
    let mut _cnt16: u16 = 1;
    let mut _cnt8: u8 = 1;

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
        block!(timer.wait()).unwrap();
        _cnt += 1;
    }
}
