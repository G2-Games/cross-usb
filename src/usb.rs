#![cfg_attr(debug_assertions, allow(async_fn_in_trait))]
use std::error::Error;
use crate::context::UsbInterface;

/// A unique USB device
pub trait Device {
    type UsbDevice;
    type UsbInterface;

    /// Open a specific interface of the device
    async fn open_interface(&self, number: u8) -> Result<UsbInterface, Box<dyn Error>>;

    /// Reset the device, which causes it to no longer be usable
    async fn reset(&self) -> Result<(), Box<dyn Error>>;

    /// 16 bit device product ID
    async fn product_id(&self) -> u16;

    /// 16 bit device vendor ID
    async fn vendor_id(&self) -> u16;

    /// Device standard class
    async fn class(&self) -> u8;

    /// Device standard subclass
    async fn subclass(&self) -> u8;

    /// Get the manufacturer string string of the device, if available without device IO
    ///
    /// Not available on Windows
    async fn manufacturer_string(&self) -> Option<String>;

    /// Get the product string of the device, if available without device IO
    async fn product_string(&self) -> Option<String>;
}

/// A specific interface of a USB device
pub trait Interface<'a> {
    /// A USB control in transfer (device to host)
    async fn control_in(&self, data: ControlIn) -> Result<Vec<u8>, Box<dyn Error>>;

    /// A USB control out transfer (host to device)
    async fn control_out(&self, data: ControlOut<'a>) -> Result<(), Box<dyn Error>>;

    /// A USB bulk in transfer (device to host)
    async fn bulk_in(&self, endpoint: u8, length: usize) -> Result<Vec<u8>, Box<dyn Error>>;

    /// A USB bulk out transfer (host to device)
    async fn bulk_out(&self, endpoint: u8, data: Vec<u8>) -> Result<usize, Box<dyn Error>>;

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
