//! Cross USB is a USB library which works seamlessly across native and WASM targets.
//!
//! The idea is the user only has to write one way to access USB devices, which can be compiled
//! to both WASM and native targets without any conditional compilation or configuration.
//!
//! ## Example:
//! ```no_run
//! # tokio_test::block_on(async {
//! use cross_usb::usb::{Device, Interface, Recipient, ControlType, ControlIn};
//!
//! // Obtain a device using its VendorID and ProductID
//! let device = cross_usb::get_device(0x054c, 0x0186).await.expect("Failed to get device");
//!
//! // Obtain an interface of the device
//! let interface = device.open_interface(0).await.expect("Failed to open interface");
//!
//! // Send a Control transfer to the device, obtaining
//! // the result and storing it in `result`, and you're done!
//! let result = interface.control_in(ControlIn {
//!         control_type: ControlType::Vendor,
//!         recipient: Recipient::Interface,
//!         request: 0x01,
//!         value: 0,
//!         index: 0,
//!         length: 4,
//!     })
//!     .await
//!     .expect("Sending control transfer failed");
//! # })
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

/// A single Device ID and Product ID pair to find when looking
/// for new USB devices using [get_device_filter]
#[doc(inline)]
pub use crate::context::FilterTuple;

/// Gets a single device from the VendorID and ProductID
#[doc(inline)]
pub use crate::context::get_device;

/// Gets a single device from a list of VendorID and ProductIDs
///
/// ## Example
/// ```no_run
/// # tokio_test::block_on(async {
/// use cross_usb::{get_device_filter, FilterTuple};
///
/// let filter = vec![
///     FilterTuple(0x054c, 0x00c9),
///     FilterTuple(0x054c, 0x0186),
/// ];
///
/// let device = get_device_filter(filter).await.expect("Could not find device in list");
/// # })
/// ```
#[doc(inline)]
pub use crate::context::get_device_filter;
