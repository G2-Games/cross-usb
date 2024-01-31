pub mod usb;

#[cfg(not(target_family = "wasm"))]
#[path = "./backend/native.rs"]
pub mod context;

#[cfg(target_family = "wasm")]
#[path = "./backend/wasm.rs"]
pub mod context;

/// Gets a single device from the VendorID and ProductID
#[doc(inline)]
pub use crate::context::get_device;
