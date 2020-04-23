// Target board: NUCLEO-F042K6

use avatar_probe_rs::open_probe;
use cortex_m::interrupt;
use stm32f0xx_hal::prelude::*;
use stm32f0xx_hal::stm32;
use std::time::Duration;
use std::thread;


fn main() {
    let interface = open_probe();
    vcell::set_memory_interface(interface);

    println!("Staring blinkey!");
    let mut dp = stm32::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.configure().freeze(&mut dp.FLASH);
    let gpiob = dp.GPIOB.split(&mut rcc);

    let mut led = interrupt::free(|cs| {
        gpiob.pb3.into_push_pull_output(cs)
    });

    loop {
        led.set_high().ok();
        thread::sleep(Duration::from_millis(500));
        led.set_low().ok();
        thread::sleep(Duration::from_millis(500));
    }
}
