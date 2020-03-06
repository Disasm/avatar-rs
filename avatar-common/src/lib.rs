#![no_std]

pub trait MemoryInterface {
    type Error;

    fn read8(&mut self, address: u32) -> Result<u8, Self::Error>;
    fn read16(&mut self, address: u32) -> Result<u16, Self::Error>;
    fn read32(&mut self, address: u32) -> Result<u32, Self::Error>;

    fn write8(&mut self, address: u32, value: u8) -> Result<(), Self::Error>;
    fn write16(&mut self, address: u32, value: u16) -> Result<(), Self::Error>;
    fn write32(&mut self, address: u32, value: u32) -> Result<(), Self::Error>;
}
