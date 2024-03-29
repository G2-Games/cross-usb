#![allow(async_fn_in_trait)]
//! This module contains the traits and associated functions and
//! structs which allow for USB communication.

use thiserror::Error;

pub trait UsbDescriptor {
    /// A unique USB Device
    type Device;

    /// Opens the USB connection, returning a [Self::Device]
    async fn open(self) -> Result<Self::Device, UsbError>;

    /// 16 bit device Product ID
    async fn product_id(&self) -> u16;

    /// 16 bit device Vendor ID
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

/// A unique USB device
pub trait UsbDevice {
    /// A unique Interface on a USB Device
    type Interface;

    /// Open a specific interface of the device
    async fn open_interface(&self, number: u8) -> Result<Self::Interface, UsbError>;

    /// Open a specific interface of the device, detaching any
    /// kernel drivers and claiming it.
    ///
    /// **Note:** This only has an effect on Native, and only on Linux.
    async fn detach_and_open_interface(&self, number: u8) -> Result<Self::Interface, UsbError>;

    /// Reset the device, which causes it to no longer be usable. You must
    /// request a new device with [crate::get_device]
    async fn reset(&self) -> Result<(), UsbError>;

    /// Remove the device from the paired devices list, causing it to no longer be usable. You must request to reconnect using [crate::get_device]
    ///
    /// **Note:** On Native this simply resets the device.
    async fn forget(&self) -> Result<(), UsbError>;

    /// 16 bit device Product ID
    async fn product_id(&self) -> u16;

    /// 16 bit device Vendor ID
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
pub trait UsbInterface<'a> {
    /// A USB control in transfer (device to host)
    /// Returns a [Result] with the bytes in a `Vec<u8>`
    async fn control_in(&self, data: ControlIn) -> Result<Vec<u8>, UsbError>;

    /// A USB control out transfer (host to device)
    async fn control_out(&self, data: ControlOut<'a>) -> Result<usize, UsbError>;

    /// A USB bulk in transfer (device to host)
    /// It takes in a bulk endpoint to send to along with the length of
    /// data to read, and returns a [Result] with the bytes
    async fn bulk_in(&self, endpoint: u8, length: usize) -> Result<Vec<u8>, UsbError>;

    /// A USB bulk out transfer (host to device).
    /// It takes in a bulk endpoint to send to along with some data as
    /// a slice, and returns a [Result] containing the number of bytes transferred
    async fn bulk_out(&self, endpoint: u8, data: &[u8]) -> Result<usize, UsbError>;

    /* TODO: Figure out interrupt transfers on Web USB

    /// A USB interrupt in transfer (device to host).
    /// Takes in an endpoint and a buffer to fill
    async fn interrupt_in(&self, endpoint: u8, length: usize) -> Result<Vec<u8>, UsbError>;

    /// A USB interrupt out transfer (host to device).
    /// Takes in an endpoint and a buffer to send
    async fn interrupt_out(&self, endpoint: u8, buf: Vec<u8>) -> Result<usize, UsbError>;
    */
}

/// An error from a USB interface
#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UsbError {
    #[error("device not found")]
    DeviceNotFound,

    #[error("device transfer failed")]
    TransferError,

    #[error("device communication failed")]
    CommunicationError(String),

    #[error("device disconnected")]
    Disconnected,

    #[error("device no longer valid")]
    Invalid,
}

/// The type of USB transfer
pub enum ControlType {
    Standard = 0,
    Class = 1,
    Vendor = 2,
}

/// The recipient of a USB transfer
pub enum Recipient {
    Device = 0,
    Interface = 1,
    Endpoint = 2,
    Other = 3,
}

/// Parameters for [UsbInterface::control_in]
pub struct ControlIn {
    pub control_type: ControlType,
    pub recipient: Recipient,
    pub request: u8,
    pub value: u16,
    pub index: u16,
    pub length: u16,
}

/// Parameters for [UsbInterface::control_out]
pub struct ControlOut<'a> {
    pub control_type: ControlType,
    pub recipient: Recipient,
    pub request: u8,
    pub value: u16,
    pub index: u16,
    pub data: &'a [u8],
}
