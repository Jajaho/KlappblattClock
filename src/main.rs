#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

use uln2003::{StepperMotor, ULN2003, Direction};

// Constants for the stepper motor
// Step angle = 0.18º/step
const STEPS_PER_REVOLUTION: i32 = 2048; // 28BYJ-48 has 4096 steps per revolution

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

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // This is the correct pin on the Raspberry Pico board. On other boards, even if they have an
    // on-board LED, it might need to be changed.
    //
    // Notably, on the Pico W, the LED is not connected to any of the RP2040 GPIOs but to the cyw43 module instead.
    // One way to do that is by using [embassy](https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/wifi_blinky.rs)
    //
    // If you have a Pico W and want to toggle a LED with a simple GPIO output pin, you can connect an external
    // LED to one of the GPIO pins, and reference that pin here. Don't forget adding an appropriate resistor
    // in series with the LED.
    let mut led_pin = pins.led.into_push_pull_output();

    // Create a timer for the ULN2003
    let timer = bsp::hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Configure the GPIO pins for the ULN2003
    let mut motor = ULN2003::new(
        pins.gpio2.into_push_pull_output(), // IN1
        pins.gpio3.into_push_pull_output(), // IN2
        pins.gpio4.into_push_pull_output(), // IN3
        pins.gpio5.into_push_pull_output(), // IN4
        Some(timer),
    );

    // Set motor speed to 15 RPM (revolutions per minute)
    let rpm: f32 = 100.0;
    
    // Calculate delay between steps
    // steps_per_minute = rpm * steps_per_revolution
    // delay_ms = (60 * 1000) / steps_per_minute
    let delay_ms = ((60.0 * 1000.0) / (rpm * STEPS_PER_REVOLUTION as f32)) as u32;

    // Set rotation direction
    motor.set_direction(Direction::Reverse);


    // run for 100 steps with 5 ms between steps
    //motor.step_for(100, 5).unwrap();

    loop {

        // Step the motor with calculated delay
        match motor.step() {
            Ok(_) => {
                // min delay tested: 1000 us
                delay.delay_us(3000);
            }
            Err(_) => {
                // Handle error - in this case we just continue
                continue;
            }
        }
    }
}