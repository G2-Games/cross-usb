//! Cross USB is a USB library which works seamlessly across native and WASM targets.
//!
//! The idea is the user only has to write one way to access USB devices, which can be compiled
//! to both WASM and native targets without any additional conditional compilation or configuration.
//!
//! For native device support, this library uses [nusb](https://docs.rs/nusb/latest/nusb/), a cross platform USB library written in Rust
//! and comparable to the very popular `libusb` C library. Web Assembly support is provided by [web-sys](https://docs.rs/web-sys/latest/web_sys/)
//! with the [Web USB API](https://developer.mozilla.org/en-US/docs/Web/API/WebUSB_API).
//!
//! When an [`Interface`] is dropped, it is automatically released.
//!
//! ### CURRENT LIMITATIONS:
//! * Hotplug support is not implemented. Waiting on [hotplug support in nusb](https://github.com/kevinmehall/nusb/pull/20).
//!
//! * Until [this pull request](https://github.com/rustwasm/wasm-bindgen/issues/3155)
//! is merged into wasm bindgen, getting a list of USB devices is not possible on WASM
//! targets. However, this isn't a huge deal as the user gets a list to select from anyway.
//!
//! ## Example:
//! ```no_run
//! # tokio_test::block_on(async {
//! use cross_usb::prelude::*;
//! use cross_usb::usb::{Recipient, ControlType, ControlIn};
//! use cross_usb::device_filter;
//!
//! // Obtain a device descriptor using a DeviceFilter,
//! // in this case with its VendorID and ProductID
//! let filters = vec![
//!     device_filter!{vendor_id: 0x054c, product_id: 0x00c9}
//! ];
//! let dev_descriptor = cross_usb::get_device(filters).await.expect("Failed to find device");
//!
//! // Open the device that the descriptor is describing
//! let dev = dev_descriptor.open().await.expect("Failed to open device");
//!
//! // Obtain an interface of the device
//! let interface = dev.open_interface(0).await.expect("Failed to open interface");
//!
//! // Send a Control transfer to the device, obtaining
//! // the result and storing it in `result`
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

/// This prelude imports all the necessary traits needed to actually use USB
/// devices and interfaces.
///
/// ```
/// use cross_usb::prelude::*;
/// ```
pub mod prelude {
    pub use crate::usb::UsbDescriptor;
    pub use crate::usb::UsbDevice;
    pub use crate::usb::UsbInterface;
}

/// The context contains the platform specific implementation of the USB transfers
#[cfg(not(target_family = "wasm"))]
#[path = "./backend/native.rs"]
mod context;

#[cfg(target_family = "wasm")]
#[path = "./backend/wasm.rs"]
mod context;

#[doc(inline)]
/// A descriptor of a USB device, containing information about a device
/// without claiming it
pub use crate::context::Descriptor;

#[doc(inline)]
/// A USB device, you must open an [`Interface`] to perform transfers
pub use crate::context::Device;

#[doc(inline)]
/// A USB interface with which to perform transfers on
pub use crate::context::Interface;

/// Information about a USB device for use in [`get_device`]
/// or [`get_device_list`]
#[doc(inline)]
pub use crate::context::DeviceFilter;

/// Gets a single (the first found) device as a [`Descriptor`] from a list of VendorID
/// and ProductIDs
///
/// ## Example
/// ```no_run
/// # tokio_test::block_on(async {
/// use cross_usb::{get_device, DeviceFilter, device_filter};
///
/// let filter = vec![
///     device_filter!{vendor_id: 0x054c, product_id: 0x00c9},
///     device_filter!{vendor_id: 0x054c},
/// ];
///
/// let device = get_device(filter).await.expect("Could not find device matching filters");
/// # })
/// ```
#[doc(inline)]
pub use crate::context::get_device;

/// Gets a list of [`Descriptor`]s from a list of VendorID and ProductIDs
///
/// ## Example
/// ```no_run
/// # tokio_test::block_on(async {
/// use cross_usb::{get_device_list, DeviceFilter, device_filter};
///
/// let filter = vec![
///     device_filter!{vendor_id: 0x054c, product_id: 0x00c9},
///     device_filter!{vendor_id: 0x054c},
/// ];
///
/// let device_list = get_device_list(filter).await.expect("Could not find device matching filters");
///
/// /* Do something with the list of devices... */
/// # })
/// ```
#[cfg(not(target_family = "wasm"))]
#[doc(inline)]
pub use crate::context::get_device_list;

/// Macro to create a device filter more easily.
///
/// The only valid keys are fields of the [`DeviceFilter`] struct.
/// You may use as many or as few of them as you need, the rest
/// of the values will be filled with [`None`].
///
/// ## Usage
/// ```
/// use cross_usb::device_filter;
///
/// // Example with all fields filled
/// device_filter!{
///     vendor_id: 0x054c,  // u16
///     product_id: 0x0186, // u16
///     class: 0xFF,        // u8
///     subclass: 0x02,     // u8
///     protocol: 0x15,     // u8
/// };
/// ```
#[macro_export]
macro_rules! device_filter {
    ($($field:ident: $val:expr),+ $(,)?) => {
        cross_usb::DeviceFilter {
            $($field: Some($val),)*
            ..cross_usb::DeviceFilter::default()
        }
    }
}
