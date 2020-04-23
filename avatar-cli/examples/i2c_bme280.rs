// Target board: NUCLEO-F042K6
//
// Connections:
//   D4 - SDA
//   D5 - SCL

use avatar_probe_rs::open_probe;
use cortex_m::interrupt;
use stm32f0xx_hal::prelude::*;
use stm32f0xx_hal::stm32;
use stm32f0xx_hal::i2c::I2c;
use std::time::Duration;
use std::thread;
use bme280::BME280;
use linux_embedded_hal::Delay;


fn main() {
    let interface = open_probe();
    vcell::set_memory_interface(interface);

    let mut p = stm32::Peripherals::take().unwrap();

    interrupt::free(|cs| {
        let mut rcc = p.RCC.configure().freeze(&mut p.FLASH);

        let gpiob = p.GPIOB.split(&mut rcc);

        // Configure pins for I2C
        let sda = gpiob.pb7.into_alternate_af1(cs);
        let scl = gpiob.pb6.into_alternate_af1(cs);

        // Configure I2C with 100kHz rate
        let i2c = I2c::i2c1(p.I2C1, (scl, sda), 100.khz(), &mut rcc);

        let mut bme280 = BME280::new_primary(i2c, Delay);

        // initialize the sensor
        bme280.init().unwrap();

        loop {
            // measure temperature, pressure, and humidity
            let measurements = bme280.measure().unwrap();

            println!();
            println!("Relative Humidity = {}%", measurements.humidity);
            println!("Temperature = {} deg C", measurements.temperature);
            println!("Pressure = {} pascals", measurements.pressure);

            thread::sleep(Duration::from_millis(1000));
        }
    });
}
