#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt::Debug;
use core::ops::{Deref, DerefMut};

pub trait MemoryInterface {
    type Error;

    fn try_read8(&mut self, address: u32) -> Result<u8, Self::Error>;
    fn try_read16(&mut self, address: u32) -> Result<u16, Self::Error>;
    fn try_read32(&mut self, address: u32) -> Result<u32, Self::Error>;

    fn try_write8(&mut self, address: u32, value: u8) -> Result<(), Self::Error>;
    fn try_write16(&mut self, address: u32, value: u16) -> Result<(), Self::Error>;
    fn try_write32(&mut self, address: u32, value: u32) -> Result<(), Self::Error>;
}

pub trait ImplementInfallible {}

pub trait InfallibleMemoryInterface {
    fn read8(&mut self, address: u32) -> u8;
    fn read16(&mut self, address: u32) -> u16;
    fn read32(&mut self, address: u32) -> u32;

    fn write8(&mut self, address: u32, value: u8);
    fn write16(&mut self, address: u32, value: u16);
    fn write32(&mut self, address: u32, value: u32);
}

impl<E, T> InfallibleMemoryInterface for T
where E: Debug, T: MemoryInterface<Error=E> + ImplementInfallible
{
    fn read8(&mut self, address: u32) -> u8 {
        self.try_read8(address).unwrap()
    }

    fn read16(&mut self, address: u32) -> u16 {
        self.try_read16(address).unwrap()
    }

    fn read32(&mut self, address: u32) -> u32 {
        self.try_read32(address).unwrap()
    }

    fn write8(&mut self, address: u32, value: u8) {
        self.try_write8(address, value).unwrap()
    }

    fn write16(&mut self, address: u32, value: u16) {
        self.try_write16(address, value).unwrap()
    }

    fn write32(&mut self, address: u32, value: u32) {
        self.try_write32(address, value).unwrap()
    }
}

pub struct StaticMemoryInterface {
    #[cfg(feature = "std")]
    pub inner: Box<dyn InfallibleMemoryInterface>,
    #[cfg(not(feature = "std"))]
    pub inner: &'static mut dyn InfallibleMemoryInterface,
}

impl Deref for StaticMemoryInterface {
    type Target = dyn InfallibleMemoryInterface;

    fn deref(&self) -> &Self::Target {
        match () {
            #[cfg(feature = "std")]
            () => self.inner.as_ref(),
            #[cfg(not(feature = "std"))]
            () => self.inner,
        }
    }
}

impl DerefMut for StaticMemoryInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match () {
            #[cfg(feature = "std")]
            () => self.inner.as_mut(),
            #[cfg(not(feature = "std"))]
            () => self.inner,
        }
    }
}
