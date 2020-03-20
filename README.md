# `avatar-rs`

> A proof-of-concept project aimed at prototyping embedded applications on a host.

## Running the examples

You will need a [NUCLEO-F042K6] board to run the examples for it.

Clone the repository:

    git clone --recursive https://github.com/Disasm/avatar-rs
    cd avatar-rs

Connect your NUCLEO-F042K6 with a USB cable.

Run the `blinky` example:

    cd avatar-cli
    cargo run --example blinky

See also [`serial_hello`] and [`i2c_bme280`] examples.

[NUCLEO-F042K6]: https://www.st.com/en/evaluation-tools/nucleo-f042k6.html
[`serial_hello`]: https://github.com/Disasm/avatar-rs/blob/master/avatar-cli/examples/serial_hello.rs
[`i2c_bme280`]: https://github.com/Disasm/avatar-rs/blob/master/avatar-cli/examples/i2c_bme280.rs
