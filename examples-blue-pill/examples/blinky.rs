// Target board: Blue Pill

use avatar_probe_rs::open_probe;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::stm32;
use embedded_hal::digital::v2::OutputPin;
use std::time::Duration;
use std::thread;


fn main() {
    let interface = open_probe();
    vcell::set_memory_interface(interface);

    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let _clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    led.set_high().ok();

    loop {
        led.set_low().ok();
        thread::sleep(Duration::from_millis(500));
        led.set_high().ok();
        thread::sleep(Duration::from_millis(500));
    }
}
