# Cross USB
[![Lib.rs Version](https://img.shields.io/crates/v/cross_usb?style=for-the-badge&logo=rust&label=lib.rs&color=%23a68bfc)](https://lib.rs/crates/cross_usb)
[![docs.rs](https://img.shields.io/docsrs/cross_usb?style=for-the-badge)](https://docs.rs/cross_usb/)

A USB library which works seamlessly across most native and WASM targets.

------------------

> [!NOTE]
> Web USB only works in Chromium based browsers for now.

> [!NOTE]
> Web USB has certain interation requirements in browsers, along with requiring
> a **Secure context**. Please read more about this on the
> [mdn web docs](https://developer.mozilla.org/en-US/docs/Web/API/WebUSB_API)

> [!IMPORTANT]
> When compiling this crate on a WASM target, you must use either the rustflags
> `RUSTFLAGS=--cfg=web_sys_unstable_apis` or by passing the argument in a
> `.cargo/config.toml` file. Read more here: https://rustwasm.github.io/wasm-bindgen/web-sys/unstable-apis.html

## Dependencies

For native USB, the crate utilizies [nusb](https://github.com/kevinmehall/nusb), a pure rust library similar to the very popular libusb.

For WASM, this crate utilizes [web-sys](https://crates.io/crates/web-sys) which gives access to browser API calls, and in this case is used to interact with [WebUSB](https://developer.mozilla.org/en-US/docs/Web/API/WebUSB_API)

## Example
To learn about how USB communciations work, check out [USB in a NutShell](https://www.beyondlogic.org/usbnutshell/usb1.shtml).

```rust
use cross_usb::prelude::*;
use cross_usb::usb::{Recipient, ControlType, ControlIn};
use cross_usb::device_filter;

// Obtain a device descriptor using a DeviceFilter,
// in this case with its VendorID and ProductID
let filters = vec![
    device_filter!{vendor_id: 0x054c, product_id: 0x00c9}
];
let dev_descriptor = cross_usb::get_device(filters).await.expect("Failed to find device");

// Open the device that the descriptor is describing
let dev = dev_descriptor.open().await.expect("Failed to open device");

// Obtain an interface of the device
let interface = dev.open_interface(0).await.expect("Failed to open interface");

// Send a Control transfer to the device, obtaining
// the result and storing it in `result`
let result = interface.control_in(ControlIn {
        control_type: ControlType::Vendor,
        recipient: Recipient::Interface,
        request: 0x01,
        value: 0,
        index: 0,
        length: 4,
    })
    .await
    .expect("Sending control transfer failed");
```
Check out the [documentation](https://docs.rs/cross_usb/latest/) as well!

## TODO

- [ ] Add choice for native backend between `libusb` wrapper and pure rust `nusb`
- [ ] Allow platform-specific operations if the user requires them
- [ ] Hot plug support... requires either using `libusb` as an optional backend or for [`nusb` to implement it](https://github.com/kevinmehall/nusb/issues/5)
