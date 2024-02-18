//! Cross USB is a USB library which works seamlessly across native and WASM targets.
//!
//! The idea is the user only has to write one way to access USB devices, which can be compiled
//! to both WASM and native targets without any additional conditional compilation or configuration.
//!
//! For native device support, this library uses [nusb](https://docs.rs/nusb/latest/nusb/), a cross platform USB library written in Rust
//! and comparable to the very popular `libusb` C library. Web Assembly support is provided by [web-sys](https://docs.rs/web-sys/latest/web_sys/)
//! with the [Web USB API](https://developer.mozilla.org/en-US/docs/Web/API/WebUSB_API).
//!
//! When a [UsbInterface] is dropped, it is automatically released.
//!
//! ## Example:
//! ```no_run
//! # tokio_test::block_on(async {
//! use cross_usb::usb::{Device, Interface, Recipient, ControlType, ControlIn};
//! use cross_usb::device_filter;
//!
//! // Obtain a device using its VendorID and ProductID
//! let filter = vec![
//!     device_filter!{vendor_id: 0x054c, product_id: 0x00c9}
//! ];
//!
//! let device = cross_usb::get_device(filter).await.expect("Failed to get device");
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

/// Information about a USB device for finding it while trying
/// to look for new USB devices using [get_device]
#[doc(inline)]
pub use crate::context::DeviceFilter;

/// Gets a single device from a list of VendorID and ProductIDs
///
/// ## Example
/// ```no_run
/// # tokio_test::block_on(async {
/// use cross_usb::{get_device, DeviceFilter, device_filter};
///
///
/// let filter = vec![
///     device_filter!{vendor_id: 0x054c, product_id: 0x00c9},
///     device_filter!{vendor_id: 0x054c},
/// ];
///
/// let device = get_device(filter).await.expect("Could not find device in list");
/// # })
/// ```
#[doc(inline)]
pub use crate::context::get_device;

/// Macro to create a device filter more easily.
///
/// The only valid keys are fields of the [DeviceFilter] struct.
/// You may use as many or as few of them as you need, the rest
/// of the values will be filled with [None].
///
/// ## Usage
/// ```
/// use cross_usb::device_filter;
///
/// // Example with all fields filled
/// device_filter!{
///     vendor_id: 0x054c,
///     product_id: 0x0186,
///     class: 0xFF,
///     subclass: 0x02,
///     protocol: 0x15,
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
