//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]
#![feature(const_for)]

use bsp::{
    entry,
    hal::gpio::{DynPinId, FunctionSio, Pin, PullDown, SioOutput},
};
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac, rtc,
    sio::Sio,
    watchdog::Watchdog,
};

mod digit;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // TODO: read initial datetime from serial
    let initial_date = rtc::DateTime {
        year: 2023,
        month: 9,
        day: 3,
        day_of_week: rtc::DayOfWeek::Sunday,
        hour: 14,
        minute: 14,
        second: 0,
    };

    let real_time_clock =
        rtc::RealTimeClock::new(pac.RTC, clocks.rtc_clock, &mut pac.RESETS, initial_date).unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // This is the correct pin on the Raspberry Pico board. On other boards, even if they have an
    // on-board LED, it might need to be changed.
    // Notably, on the Pico W, the LED is not connected to any of the RP2040 GPIOs but to the cyw43 module instead. If you have
    // a Pico W and want to toggle a LED with a simple GPIO output pin, you can connect an external
    // LED to one of the GPIO pins, and reference that pin here.
    let mut led_pin = pins.led.into_push_pull_output();

    // Rows are GPIO 0-12
    let mut row_pins = [
        pins.gpio0.into_push_pull_output().into_dyn_pin(),
        pins.gpio1.into_push_pull_output().into_dyn_pin(),
        pins.gpio2.into_push_pull_output().into_dyn_pin(),
        pins.gpio3.into_push_pull_output().into_dyn_pin(),
        pins.gpio4.into_push_pull_output().into_dyn_pin(),
        pins.gpio5.into_push_pull_output().into_dyn_pin(),
        pins.gpio6.into_push_pull_output().into_dyn_pin(),
        pins.gpio7.into_push_pull_output().into_dyn_pin(),
        pins.gpio8.into_push_pull_output().into_dyn_pin(),
        pins.gpio9.into_push_pull_output().into_dyn_pin(),
        pins.gpio10.into_push_pull_output().into_dyn_pin(),
        pins.gpio11.into_push_pull_output().into_dyn_pin(),
        pins.gpio12.into_push_pull_output().into_dyn_pin(),
    ];

    // Columns are pins 19-22
    let mut col_pins = [
        pins.gpio19.into_push_pull_output().into_dyn_pin(),
        pins.gpio20.into_push_pull_output().into_dyn_pin(),
        pins.gpio21.into_push_pull_output().into_dyn_pin(),
        pins.gpio22.into_push_pull_output().into_dyn_pin(),
    ];

    // now_digits is a 4-digit representation of the current time written to in the
    // main loop and read from in the print_digits interrupt.
    let mut now_digits: [usize; 4] = [0, 0, 0, 0];

    loop {
        let now = real_time_clock.now().unwrap();
        now_digits[0] = now.hour as usize / 10;
        now_digits[1] = now.hour as usize % 10;
        now_digits[2] = now.minute as usize / 10;
        now_digits[3] = now.minute as usize % 10;

        info!("on!");
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        info!("off!");
        led_pin.set_low().unwrap();
        delay.delay_ms(500);

        // TODO: print multiple digits
        // for (k, channel) in channels.iter().enumerate() {
        let k: usize = 0;
        let char_for_digit = now_digits[k];

        if let Some(digit) = digit::DIGITS.get(char_for_digit - 1) {
            print_digit(digit, &mut row_pins, &mut col_pins, &mut delay, 100, 0.1);
        }
    }
}

type MyPin = Pin<DynPinId, FunctionSio<SioOutput>, PullDown>;

// print_digit drives the row and column pins to display the given digit for
// duration_us.
fn print_digit(
    digit: &digit::DigitArray,
    row_pins: &mut [MyPin; digit::NUM_ROWS],
    col_pins: &mut [MyPin; digit::NUM_COLS],
    delay: &mut cortex_m::delay::Delay,
    duration_us: u32,
    brightness: f32,
) {
    let duration_per_frame_ns = duration_us * 1000 / digit::NUM_COLS as u32;
    let duration_on_per_frame_ns = (duration_per_frame_ns as f32 * brightness) as u32;
    let duration_on_per_frame_us = duration_on_per_frame_ns / 1000;
    let duration_off_per_frame_us = (duration_per_frame_ns - duration_on_per_frame_ns) / 1000;

    for (j, col_pin) in col_pins.iter_mut().enumerate() {
        let mut num_rows_lit = 0;
        // Set the relevant row pins high for the column
        let digit_col = digit[j];
        for (digit_i, val) in digit_col.iter().enumerate() {
            if *val == 1 {
                row_pins[digit_i].set_high().unwrap();
                num_rows_lit += 1;
            }
        }
        if num_rows_lit > 0 {
            col_pin.set_high().unwrap();
        }
        delay.delay_us(duration_on_per_frame_us);

        // Set all pins low
        col_pin.set_low().unwrap();
        for row_pin in row_pins.iter_mut() {
            row_pin.set_low().unwrap();
        }
        delay.delay_us(duration_off_per_frame_us);
    }
}
