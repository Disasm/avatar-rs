// Target board: NUCLEO-F042K6
//
// Serial is connected to the on-board ST-LINK

use avatar_probe_rs::open_probe;
use cortex_m::interrupt;
use stm32f0xx_hal::prelude::*;
use stm32f0xx_hal::stm32;
use stm32f0xx_hal::serial::Serial;
use core::fmt::Write;
use std::time::Duration;
use std::thread;
use stm32f0xx_hal::rcc::HSEBypassMode;


fn main() {
    let interface = open_probe();
    vcell::set_memory_interface(interface);

    let mut dp = stm32::Peripherals::take().unwrap();

    interrupt::free(|cs| {
        let mut rcc = dp.RCC.configure().hse(8.mhz(), HSEBypassMode::Bypassed).sysclk(8.mhz()).freeze(&mut dp.FLASH);

        let gpioa = dp.GPIOA.split(&mut rcc);
        let tx = gpioa.pa2.into_alternate_af1(cs);
        let rx = gpioa.pa15.into_alternate_af1(cs);

        let mut serial = Serial::usart2(dp.USART2, (tx, rx), 115_200.bps(), &mut rcc);

        write!(serial, "Hello, world!\r\n").unwrap();

        loop {
            thread::sleep(Duration::from_millis(500));
            write!(serial, "Hello again!\r\n").unwrap();
        }
    });
}
