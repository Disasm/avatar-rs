// Target board: DWM1001-DEV

use avatar_probe_rs::open_probe;
use nrf52832_hal::nrf52832_pac as pac;
use nrf52832_hal::gpio;
use nrf52832_hal::gpio::p0::*;
use nrf52832_hal::gpio::Level;
use nrf52832_hal::gpio::*;
use embedded_hal::digital::v2::OutputPin;
use std::time::Duration;
use std::thread;


fn main() {
    let interface = open_probe();
    vcell::set_memory_interface(interface);

    let p = pac::Peripherals::take().unwrap();
    let port0 = p.P0.split();

    let mut led1: P0_30<gpio::Output<PushPull>> = port0.p0_30.into_push_pull_output(Level::High);

    loop {
        led1.set_high().ok();
        thread::sleep(Duration::from_millis(100));
        led1.set_low().ok();
        thread::sleep(Duration::from_millis(100));
    }
}
