// Target board: NUCLEO-F042K6

use avatar_probe_rs::open_probe;
use cortex_m::interrupt;
use stm32f0xx_hal::prelude::*;
use stm32f0xx_hal::stm32;
use std::time::Duration;
use std::thread;
use stm32f0xx_hal::gpio::{
    Output, PushPull,
    gpioa::*,
    gpiob::*,
};

macro_rules! pin_set {
    ($value:expr, $mask:expr, $pin:expr) => {
        if ($value & $mask) != 0 {
            $pin.set_high().ok();
        } else {
            $pin.set_low().ok();
        }
    }
}

struct HPDL {
    d0: PA3<Output<PushPull>>, // A2
    d1: PA4<Output<PushPull>>, // A3
    d2: PA5<Output<PushPull>>, // A4
    d3: PA6<Output<PushPull>>, // A5
    d4: PA8<Output<PushPull>>, // D9
    d5: PB1<Output<PushPull>>, // D6
    d6: PA7<Output<PushPull>>, // A6
    a0: PB4<Output<PushPull>>, // D12
    a1: PB5<Output<PushPull>>, // D11
    nwr: PA11<Output<PushPull>>, // D10
}

impl HPDL {
    pub fn init(&mut self) {
        self.nwr.set_high();
    }

    pub fn write(&mut self, addr: u8, byte: u8) {
        pin_set!(addr, 0b01, self.a0);
        pin_set!(addr, 0b10, self.a1);

        pin_set!(byte, 0b0000001, self.d0);
        pin_set!(byte, 0b0000010, self.d1);
        pin_set!(byte, 0b0000100, self.d2);
        pin_set!(byte, 0b0001000, self.d3);
        pin_set!(byte, 0b0010000, self.d4);
        pin_set!(byte, 0b0100000, self.d5);
        pin_set!(byte, 0b1000000, self.d6);

        self.nwr.set_low().ok();
        self.nwr.set_high().ok();
    }

    pub fn write_str(&mut self, s: &str) {
        let s = s.to_ascii_uppercase();
        let bytes = s.as_bytes();

        for i in 0..4 {
            if i < bytes.len() {
                self.write(3 - i as u8, bytes[i]);
            } else {
                self.write(3 - i as u8, 0);
            }
        }
    }
}

fn main() {
    let interface = open_probe();
    vcell::set_memory_interface(interface);

    let mut dp = stm32::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.configure().freeze(&mut dp.FLASH);

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    let cs = unsafe { core::mem::zeroed() };
    let mut hpdl = HPDL {
        d5: gpiob.pb1.into_push_pull_output(&cs), // D6
        d4: gpioa.pa8.into_push_pull_output(&cs), // D9
        nwr: gpioa.pa11.into_push_pull_output(&cs), // D10
        a1: gpiob.pb5.into_push_pull_output(&cs), // D11
        a0: gpiob.pb4.into_push_pull_output(&cs), // D12

        d6: gpioa.pa7.into_push_pull_output(&cs), // A6
        d3: gpioa.pa6.into_push_pull_output(&cs), // A5
        d2: gpioa.pa5.into_push_pull_output(&cs), // A4
        d1: gpioa.pa4.into_push_pull_output(&cs), // A3
        d0: gpioa.pa3.into_push_pull_output(&cs), // A2
    };
    hpdl.init();

    let args: Vec<String> = std::env::args().collect();
    let s = "     ".to_string() + &args.get(1).unwrap_or(&"test test".to_string());

    let mut offset = 0;
    loop {
        hpdl.write_str(&s[offset..]);
        offset = offset + 1;
        if offset >= s.len() {
            offset = 0;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
