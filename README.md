# `avatar-rs`

> A proof-of-concept project aimed at prototyping embedded applications on a host.

## Running the examples

You will need a [NUCLEO-F042K6] board to run the examples for it.

Clone the repository:

    git clone --recursive https://github.com/Disasm/avatar-rs
    cd avatar-rs

Connect your NUCLEO-F042K6 with a USB cable.

Run the `blinky` example:

    cd examples-nucleo-f042k6
    cargo run --example blinky

See also [`serial_hello`] and [`i2c_bme280`] examples.

[NUCLEO-F042K6]: https://www.st.com/en/evaluation-tools/nucleo-f042k6.html
[`serial_hello`]: https://github.com/Disasm/avatar-rs/blob/master/examples-nucleo-f042k6/examples/serial_hello.rs
[`i2c_bme280`]: https://github.com/Disasm/avatar-rs/blob/master/examples-nucleo-f042k6/examples/i2c_bme280.rs


## Porting to the different families

Peripheral access crates don't require any changes if generated with svd2rust 0.16.1

HAL crates require changes if they use the following constructions.

### `volatile_read`/`volatile_write`

All the volatile memory accesses should be replaced with calls to the `VolatileCell` methods.

For example,

    unsafe { ptr::read_volatile(&self.spi.dr as *const _ as *const u8) }

should be replaced with

    unsafe { (*(&self.spi.dr as *const _ as *const vcell::VolatileCell<u8>)).get() }

and

    unsafe { ptr::write_volatile(&self.spi.dr as *const _ as *mut u8, byte) }

should be replaced with

    unsafe { (*(&self.spi.dr as *const _ as *const vcell::VolatileCell<u8>)).set(byte) }
