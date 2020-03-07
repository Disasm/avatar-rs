// Target board: NUCLEO-F042K6

use avatar_probe_rs::AvatarProbe;
use avatar_common::StaticMemoryInterface;
use stm32f0xx_hal::prelude::*;
use stm32f0xx_hal::stm32;
use std::time::Duration;
use std::thread;

mod interrupt {
    use cortex_m::interrupt::CriticalSection;

    pub fn free<F, R>(f: F) -> R
        where
            F: FnOnce(&CriticalSection) -> R,
    {
        let cs: CriticalSection = unsafe { std::mem::zeroed() };

        f(&cs)
    }
}

fn main() {
    let interface = match AvatarProbe::open_any() {
        Ok(probe) => probe,
        Err(e) => {
            println!("Can't open probe: {:?}", e);
            return;
        }
    };

    let interface = Box::new(interface);

    static mut INTERFACE: Option<StaticMemoryInterface> = None;

    unsafe {
        INTERFACE.replace(StaticMemoryInterface {
            inner: interface
        });
    }

    let interface = unsafe { &mut INTERFACE }.as_mut().unwrap();
    vcell::set_memory_interface(interface);

    println!("Staring blinkey!");
    let mut dp = unsafe { stm32::Peripherals::steal() };
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
