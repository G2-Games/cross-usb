//! Cross USB is a USB library which works seamlessly across native and WASM targets.
//!
//! The idea is the user only has to write one way to access USB devices, which can be compiled
//! to both WASM and native targets without any conditional compilation or configuration.
//!
//! ## Example:
//! ```no_run
//! use cross_usb::usb::{Device, Recipient, ControlType, ControlIn};
//!
//! // Obtain a device using its VendorID and ProductID
//! let device = cross_usb::get_device(0x054c, 0x0186).await.expect("");
//!
//! // Obtain an interface of the device
//! let interface = usb_device.open_interface(0).await.expect("Failed to open interface");
//!
//! // Send a Control transfer to the device, obtaining
//! // the result and storing it in `result`, and you're done!
//! let result = match interface.control_in(ControlIn {
//!         control_type: ControlType::Vendor,
//!         recipient: Recipient::Interface,
//!         request: 0x01,
//!         value: 0,
//!         index: 0,
//!         length: 4,
//!     })
//!     .await
//!     .expect("Sending control transfer failed");
//! ```
pub mod usb;

#[cfg(not(target_family = "wasm"))]
#[path = "./backend/native.rs"]
/// The context contains the platform specific implementation of the USB transfers
mod context;

#[cfg(target_family = "wasm")]
#[path = "./backend/wasm.rs"]
/// The context contains the platform specific implementation of the USB transfers
mod context;

#[doc(inline)]
/// An implementation of a USB device
pub use crate::context::UsbDevice;

#[doc(inline)]
/// An implementation of a USB interface
pub use crate::context::UsbInterface;

/// Gets a single device from the VendorID and ProductID
#[doc(inline)]
pub use crate::context::get_device;
