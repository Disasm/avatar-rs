use usb_device::class_prelude::*;
use usb_device::Result;
use core::marker::PhantomData;
use usb_device::device::{UsbDeviceBuilder, UsbVidPid, UsbDevice};
use core::convert::TryInto;

pub const VID: u16 = 0x16c0;
pub const PID: u16 = 0x05dc;
pub const MANUFACTURER: &'static str = "AvatarClass Manufacturer";
pub const PRODUCT: &'static str = "avatar-rs USB class";
pub const SERIAL_NUMBER: &'static str = "AvatarClass Serial";

pub const REQ_READ8: u8 = 0x01;
pub const REQ_WRITE8: u8 = 0x01;
pub const REQ_READ16: u8 = 0x02;
pub const REQ_WRITE16: u8 = 0x02;
pub const REQ_READ32: u8 = 0x04;
pub const REQ_WRITE32: u8 = 0x04;


pub struct AvatarClass<'a, B: UsbBus> {
    interface: InterfaceNumber,
    read_ep: EndpointOut<'a, B>,
    write_ep: EndpointIn<'a, B>,
    _marker: PhantomData<B>
}

impl<B: UsbBus> AvatarClass<'_, B> {
    pub fn new(alloc: &UsbBusAllocator<B>) -> AvatarClass<B> {
        AvatarClass {
            interface: alloc.interface(),
            read_ep: alloc.bulk(16),
            write_ep: alloc.bulk(16),
            _marker: PhantomData
        }
    }

    /// Convenience method to create a UsbDevice that is configured correctly for AvatarClass.
    pub fn make_device<'a, 'b>(&'a self, usb_bus: &'b UsbBusAllocator<B>, serial: Option<&'static str>) -> UsbDevice<'b, B> {
        let serial = serial.unwrap_or(SERIAL_NUMBER);
        UsbDeviceBuilder::new(&usb_bus, UsbVidPid(VID, PID))
            .manufacturer(MANUFACTURER)
            .product(PRODUCT)
            .serial_number(serial)
            .build()
    }
}

impl<B: UsbBus> UsbClass<B> for AvatarClass<'_, B> {
    fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
        writer.interface(self.interface, 0xff, 0, 0)?;
        writer.endpoint(&self.write_ep)?;
        writer.endpoint(&self.read_ep)?;
        Ok(())
    }

    fn endpoint_out(&mut self, addr: EndpointAddress) {
        if addr == self.read_ep.address() {
            let mut buf = [0; 16];
            let size = self.read_ep.read(&mut buf).unwrap();

            self.read_ep.stall();

            if size < 5 {
                return;
            }

            let command = buf[0];
            let address = u32::from_le_bytes(buf[1..5].try_into().unwrap()) as usize;

            let payload_size = size - 1 - 4;
            let payload = &buf[5..];

            unsafe {
                match (payload_size, command) {
                    (0, REQ_READ8) => {
                        let value = (address as * const u8).read_volatile();
                        self.write_ep.write(&[value]).ok();
                    }
                    (0, REQ_READ16) => {
                        let value = (address as *const u16).read_volatile();
                        self.write_ep.write(&value.to_le_bytes()).ok();
                    }
                    (0, REQ_READ32) => {
                        let value = (address as *const u32).read_volatile();
                        self.write_ep.write(&value.to_le_bytes()).ok();
                    }
                    (1, REQ_WRITE8) => {
                        (address as *mut u8).write_volatile(payload[0]);
                        self.read_ep.unstall();
                    }
                    (2, REQ_WRITE16) => {
                        let value = u16::from_le_bytes(payload[..2].try_into().unwrap());
                        (address as *mut u16).write_volatile(value);
                        self.read_ep.unstall();
                    }
                    (4, REQ_WRITE32) => {
                        let value = u32::from_le_bytes(payload[..4].try_into().unwrap());
                        (address as *mut u32).write_volatile(value);
                        self.read_ep.unstall();
                    }
                    _ => return,
                }
            }
        }
    }

    fn endpoint_in_complete(&mut self, addr: EndpointAddress) {
        if addr == self.write_ep.address() {
            self.read_ep.unstall();
        }
    }
}
