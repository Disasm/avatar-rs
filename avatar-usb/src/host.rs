use std::time::Duration;
use libusb::*;
use crate::class;

pub const TIMEOUT: Duration = Duration::from_secs(1);
pub const EN_US: u16 = 0x0409;


pub struct AvatarDevice<'a> {
    pub handle: DeviceHandle<'a>,
}

impl AvatarDevice<'_> {
    pub fn open(ctx: &Context) -> Result<AvatarDevice<'_>> {
        for device in ctx.devices()?.iter() {
            let device_descriptor = device.device_descriptor()?;

            if !(device_descriptor.vendor_id() == class::VID
                && device_descriptor.product_id() == class::PID) {
                continue;
            }

            let handle = device.open()?;

            let languages = handle.read_languages(TIMEOUT)?;
            if languages.len() == 0 || languages[0].lang_id() != EN_US {
                continue;
            }

            let product = handle.read_product_string(languages[0], &device_descriptor, TIMEOUT)?;
            if product == class::PRODUCT {
                return Ok(AvatarDevice {
                    handle,
                });
            }
        }

        Err(libusb::Error::NoDevice)
    }
}
