use avatar_common::{MemoryInterface, ImplementInfallible, StaticMemoryInterface};
use probe_rs::{Probe, Error, Session, Memory};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct AvatarProbe {
    _session: Session,
    memory: Memory,
}

impl AvatarProbe {
    pub fn open_any() -> Result<Self, Error> {
        let probes = Probe::list_all();
        if probes.len() == 0 {
            return Err(Error::UnableToOpenProbe("Probe not found"));
        }

        let probe = probes[0].open()?;

        Self::open(probe)
    }

    pub fn open(probe: Probe) -> Result<Self, Error> {
        let session: Session = probe.attach("stm32f401")?;

        // Select a core.
        let core = session.attach_to_core(0)?;
        // Reset and halt the attached core.
        core.reset_and_halt()?;

        let memory = session.attach_to_memory(0)?;
        Ok(Self{
            _session: session,
            memory
        })
    }
}

impl MemoryInterface for AvatarProbe {
    type Error = Error;

    fn try_read8(&mut self, address: u32) -> Result<u8, Error> {
        self.memory.read8(address)
    }

    fn try_read16(&mut self, address: u32) -> Result<u16, Error> {
        // TODO: fix this

        let value: u32 = self.memory.read32(address & !0b11)?;

        let value16 = if address & 0b10 == 0b00 {
            (value >> 16) as u16
        } else {
            (value & 0xffff) as u16
        };
        Ok(value16)
    }

    fn try_read32(&mut self, address: u32) -> Result<u32, Error> {
        self.memory.read32(address)
    }

    fn try_read_block32(&mut self, address: u32, data: &mut [u32]) -> Result<(), Error> {
        self.memory.read_block32(address, data)
    }

    fn try_write8(&mut self, address: u32, value: u8) -> Result<(), Error> {
        self.memory.write8(address, value)
    }

    fn try_write16(&mut self, address: u32, value: u16) -> Result<(), Error> {
        // TODO: fix this

        let old_value: u32 = self.memory.read32(address & !0b11)?;
        let new_value = if address & 0b10 == 0b00 {
            (old_value & 0xffff_0000) | (value as u32)
        } else {
            (old_value & 0x0000_ffff) | ((value as u32) << 16)
        };
        self.memory.write32(address & !0b11, new_value)
    }

    fn try_write32(&mut self, address: u32, value: u32) -> Result<(), Error> {
        self.memory.write32(address, value)
    }

    fn try_write_block32(&mut self, address: u32, data: &[u32]) -> Result<(), Error> {
        self.memory.write_block32(address, data)
    }
}

impl ImplementInfallible for AvatarProbe {}


pub fn open_probe() -> &'static mut StaticMemoryInterface {
    static TAKEN: AtomicBool = AtomicBool::new(false);

    if TAKEN.compare_and_swap(false, true, Ordering::SeqCst) {
        panic!("Probe is already opened");
    }

    let interface = match AvatarProbe::open_any() {
        Ok(probe) => probe,
        Err(e) => {
            panic!("Can't open probe: {:?}", e);
        }
    };

    let interface = Box::new(interface);

    static mut INTERFACE: Option<StaticMemoryInterface> = None;

    unsafe {
        INTERFACE.replace(StaticMemoryInterface {
            inner: interface
        });
    }

    unsafe { &mut INTERFACE }.as_mut().unwrap()
}
