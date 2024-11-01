#![no_std]
#![no_main]

use panic_halt as _;
use rp_pico::{
    hal::{self, Clock},
    pac,
    entry,
};
use uln2003::{StepperMotor, ULN2003, Direction};

// Constants for the stepper motor
const STEPS_PER_REVOLUTION: i32 = 4096; // 28BYJ-48 has 4096 steps per revolution

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Create a delay abstraction based on SysTick
    let mut delay = cortex_m::delay::Delay::new(
        core.SYST, 
        clocks.system_clock.freq().to_Hz()
    );

    // Set up the GPIO pins
    let sio = hal::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Create a timer for the ULN2003
    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Configure the GPIO pins for the ULN2003
    let mut motor = ULN2003::new(
        pins.gpio2.into_push_pull_output(), // IN1
        pins.gpio3.into_push_pull_output(), // IN2
        pins.gpio4.into_push_pull_output(), // IN3
        pins.gpio5.into_push_pull_output(), // IN4
        Some(timer),
    );

    // Set motor speed to 15 RPM (revolutions per minute)
    let rpm: f32 = 15.0;
    
    // Calculate delay between steps
    // steps_per_minute = rpm * steps_per_revolution
    // delay_ms = (60 * 1000) / steps_per_minute
    let delay_ms = ((60.0 * 1000.0) / (rpm * STEPS_PER_REVOLUTION as f32)) as u32;

    // Set rotation direction
    motor.set_direction(Direction::Normal);

    loop {
        // Step the motor with calculated delay
        match motor.step() {
            Ok(_) => {
                delay.delay_ms(delay_ms);
            }
            Err(_) => {
                // Handle error - in this case we just continue
                continue;
            }
        }
    }
}