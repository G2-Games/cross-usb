use std::error::Error;
use nusb;

use crate::usb::{ControlIn, ControlOut, ControlType, Device, Interface, Recipient};

pub struct UsbDevice {
    device_info: nusb::DeviceInfo,
    device: nusb::Device,
}

pub struct UsbInterface {
    interface: nusb::Interface,
}

/// Gets a single device from the VendorID and ProductID
pub async fn get_device(vendor_id: u16, product_id: u16) -> Result<UsbDevice, Box<dyn Error>> {
    let devices = nusb::list_devices().unwrap();

    let mut device_info = None;
    for device in devices {
        if device.vendor_id() == vendor_id && device.product_id() == product_id {
            device_info = Some(device);
            break;
        }
    }

    let device_info = match device_info {
        Some(dev) => dev,
        None => return Err("Device not found".into()),
    };

    let device = device_info.open()?;

    Ok(UsbDevice {
        device_info,
        device
    })
}

impl Device for UsbDevice {
    type UsbDevice = UsbDevice;
    type UsbInterface = UsbInterface;

    async fn open_interface(&self, number: u8) -> Result<UsbInterface, Box<dyn Error>> {
        let interface = match self.device.claim_interface(number) {
            Ok(inter) => inter,
            Err(e) => return Err(e.into()),
        };

        Ok(UsbInterface { interface })
    }

    async fn reset(&self) -> Result<(), Box<dyn Error>> {
        match self.device.reset() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into())
        }
    }

    async fn vendor_id(&self) -> u16 {
        self.device_info.vendor_id()
    }

    async fn product_id(&self) -> u16 {
        self.device_info.product_id()
    }
}

impl<'a> Interface<'a> for UsbInterface {
    async fn control_in(&self, data: ControlIn) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(self.interface.control_in(data.into()).await.into_result()?)
    }

    async fn control_out(&self, data: ControlOut<'a>) -> Result<(), Box<dyn Error>> {
        match self.interface.control_out(data.into()).await.into_result() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into())
        }
    }

    async fn bulk_in(&self, endpoint: u8, length: usize) -> Result<Vec<u8>, Box<dyn Error>> {
        let request_buffer = nusb::transfer::RequestBuffer::new(length);

        Ok(self.interface.bulk_in(endpoint, request_buffer).await.into_result()?)
    }

    async fn bulk_out(&self, endpoint: u8, data: Vec<u8>) -> Result<usize, Box<dyn Error>> {
        match self.interface.bulk_out(endpoint, data).await.into_result() {
            Ok(len) => Ok(len.actual_length()),
            Err(e) => Err(e.into())
        }
    }
}

impl From<ControlIn> for nusb::transfer::ControlIn {
    fn from(val: ControlIn) -> Self {
        nusb::transfer::ControlIn {
            control_type: val.control_type.into(),
            recipient: val.recipient.into(),
            request: val.request,
            value: val.value,
            index: val.index,
            length: val.length,
        }
    }
}

impl<'a> From<ControlOut<'a>> for nusb::transfer::ControlOut<'a> {
    fn from(val: ControlOut<'a>) -> Self {
        nusb::transfer::ControlOut {
            control_type: val.control_type.into(),
            recipient: val.recipient.into(),
            request: val.request,
            value: val.value,
            index: val.index,
            data: val.data,
        }
    }
}

impl From<ControlType> for nusb::transfer::ControlType {
    fn from(val: ControlType) -> Self {
        match val {
            ControlType::Standard => nusb::transfer::ControlType::Standard,
            ControlType::Class => nusb::transfer::ControlType::Class,
            ControlType::Vendor => nusb::transfer::ControlType::Vendor,
        }
    }
}

impl From<Recipient> for nusb::transfer::Recipient {
    fn from(val: Recipient) -> Self {
        match val {

            Recipient::Device => nusb::transfer::Recipient::Device,
            Recipient::Interface => nusb::transfer::Recipient::Interface,
            Recipient::Endpoint => nusb::transfer::Recipient::Endpoint,
            Recipient::Other => nusb::transfer::Recipient::Other,
        }
    }
}
