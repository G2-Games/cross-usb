# Cross USB

An attempt at a USB library which works seamlessly across native and WASM targets.

[Crates.io](https://crates.io/crates/cross_usb)

[Documentation](https://docs.rs/cross_usb/)

------------------

> [!NOTE]  
> Web USB only works in Chromium based browsers for now.

## Dependencies

For native USB, the crate utilizies [nusb](https://github.com/kevinmehall/nusb), a pure rust library similar to the very popular libusb.

For WASM, this crate utilizes [web-sys](https://crates.io/crates/web-sys) which gives access to browser API calls, and in this case is used to interact with [WebUSB](https://developer.mozilla.org/en-US/docs/Web/API/WebUSB_API)
