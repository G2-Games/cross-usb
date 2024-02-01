#![allow(async_fn_in_trait)]
//! This module contains the traits and associated functions and
//! structs which allow for USB communication.

use crate::context::UsbInterface;
use thiserror::Error;

/// A unique USB device
pub trait Device {
    /// A unique USB Device
    type UsbDevice;

    /// A unique Interface on a USB Device
    type UsbInterface;

    /// Open a specific interface of the device
    async fn open_interface(&self, number: u8) -> Result<UsbInterface, UsbError>;

    /// Reset the device, which causes it to no longer be usable
    async fn reset(&self) -> Result<(), UsbError>;

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
pub trait Interface<'a> {
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

    /* Interrupt transfers are a work in progress
    async fn interrupt_in(&self, _endpoint: u8, _buf: Vec<u8>) {
        unimplemented!()
    }

    async fn interrupt_out(&self, _endpoint: u8, _buf: Vec<u8>) {
        unimplemented!()
    }
    */
}

/// An error from a USB interface
#[derive(Error, Debug)]
pub enum UsbError {
    #[error("device not found")]
    DeviceNotFound,

    #[error("device transfer failed")]
    TransferError,

     #[error("device communication failed")]
    CommunicationError,

    #[error("device disconnected")]
    Disconnected,
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

/// Parameters for [Interface::control_in]
pub struct ControlIn {
    pub control_type: ControlType,
    pub recipient: Recipient,
    pub request: u8,
    pub value: u16,
    pub index: u16,
    pub length: u16,
}

/// Parameters for [Interface::control_out]
pub struct ControlOut<'a> {
    pub control_type: ControlType,
    pub recipient: Recipient,
    pub request: u8,
    pub value: u16,
    pub index: u16,
    pub data: &'a [u8],
}
