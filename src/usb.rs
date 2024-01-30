use std::error::Error;

use crate::context::UsbInterface;

/// A unique USB device
pub trait Device {
    type UsbDevice;
    type UsbInterface;

    async fn open_interface(&self, number: u8) -> Result<UsbInterface, Box<dyn Error>>;
    async fn reset(&self) -> Result<(), Box<dyn Error>>;

    //TODO: Implement these placeholders
    async fn product_id(&self) -> u16 {
        0x00
    }
    async fn vendor_id(&self) -> u16 {
        0x00
    }
    async fn class(&self) -> u16 {
        0x00
    }
    async fn subclass(&self) -> u16 {
        0x00
    }
    async fn manufacturer_string(&self) -> Option<&str> {
        None
    }
    async fn product_string(&self) -> Option<&str> {
        None
    }
}

/// A specific interface of a USB device
pub trait Interface<'a> {
    async fn control_in(&self, data: ControlIn) -> Result<Vec<u8>, Box<dyn Error>>;
    async fn control_out(&self, data: ControlOut<'a>) -> Result<(), Box<dyn Error>>;

    async fn bulk_in(&self, endpoint: u8, buf: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>>;
    async fn bulk_out(&self, endpoint: u8, buf: Vec<u8>) -> Result<(), Box<dyn Error>>;

    async fn interrupt_in(&self, _endpoint: u8, _buf: Vec<u8>) {
        unimplemented!()
    }

    async fn interrupt_out(&self, _endpoint: u8, _buf: Vec<u8>) {
        unimplemented!()
    }
}

pub enum ControlType {
    Standard = 0,
    Class = 1,
    Vendor = 2,
}

pub enum Recipient {
    Device = 0,
    Interface = 1,
    Endpoint = 2,
    Other = 3,
}

pub struct ControlIn {
    pub control_type: ControlType,
    pub recipient: Recipient,
    pub request: u8,
    pub value: u16,
    pub index: u16,
    pub length: u16,
}

pub struct ControlOut<'a> {
    pub control_type: ControlType,
    pub recipient: Recipient,
    pub request: u8,
    pub value: u16,
    pub index: u16,
    pub data: &'a[u8],
}
