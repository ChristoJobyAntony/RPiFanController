# RPiFanController
**A Fan Controller for Raspberry PI developed using Rust and rppal**

**Utliizes the RPI-4 PWM enabled GPIO pin (BCM 18), to control  fan speed based on cpu temperature**

### Features
- Support for cleaning-up GPIO pins after unexpected shutdowns.
- Configurable fan curve, with multiple points .
- Hardware enable PWM

### Usage
1. Install Rust [link](https://www.rust-lang.org/tools/install)
2. Clone the repositary
3. Change the constants in main.rs if required
3. Build the project using `cargo build`
4. Run the the the binary   

