# Raspberry Pi Pico Stepper Motor Controller

This project provides a Rust implementation for controlling a 28BYJ-48 stepper motor with a ULN2003 driver board using a Raspberry Pi Pico. The motor speed can be set in revolutions per minute (RPM).

## Hardware Requirements

- Raspberry Pi Pico
- 28BYJ-48 Stepper Motor
- ULN2003 Driver Board
- 5V power supply for the motor (can be from Pico's VBUS)

## Connections

Connect the ULN2003 driver board to the Raspberry Pi Pico as follows:

```
ULN2003 -> Pico
IN1     -> GPIO2
IN2     -> GPIO3
IN3     -> GPIO4
IN4     -> GPIO5
VCC     -> VBUS (5V)
GND     -> GND
```

## Software Requirements

Before building the project, you need to have the following installed:

1. Rust toolchain:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Target for ARM Cortex-M0+:
```bash
rustup target add thumbv6m-none-eabi
```

3. UF2 conversion tool:
```bash
cargo install elf2uf2-rs
```

## Project Setup

1. Clone this repository:
```bash
git clone <repository-url>
cd pico-stepper
```

2. Verify the project structure:
```
pico-stepper/
├── .cargo/
│   └── config.toml        # Cargo configuration for ARM target
├── src/
│   └── main.rs           # Main program code
├── build.rs              # Build script for memory configuration
├── memory.x             # Memory layout for the RP2040
├── README.md            # This file
└── Cargo.toml           # Project dependencies and configuration
```

## Building and Flashing

1. Build the project:
```bash
cargo build --release
```

2. The UF2 file will be created at:
```
target/thumbv6m-none-eabi/release/flipclock.uf2
```

3. Flash to the Pico:
   - Hold the BOOTSEL button on the Pico while connecting it to your computer
   - Release the button once connected
   - The Pico should appear as a USB mass storage device
   - Copy the .uf2 file to the Pico drive
   - The Pico will automatically restart and run the program

## Configuration

The motor speed can be adjusted by changing the `rpm` value in `src/main.rs`:

```rust
// Set motor speed to 15 RPM (revolutions per minute)
let rpm: f32 = 15.0;
```

The 28BYJ-48 motor has:
- 4096 steps per revolution (in half-step mode, which this code uses)
- Gear ratio of 64:1
- Maximum practical speed of about 15-20 RPM

## Troubleshooting

1. **Build fails:**
   - Ensure all required tools are installed
   - Verify the memory.x file exists in the project root

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under MIT License.
