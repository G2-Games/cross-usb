# Cross USB

A USB library which works seamlessly across most native and WASM targets.

[Crates.io](https://crates.io/crates/cross_usb)

[Documentation](https://docs.rs/cross_usb/)

------------------

> [!NOTE]  
> Web USB only works in Chromium based browsers for now.

## Dependencies

For native USB, the crate utilizies [nusb](https://github.com/kevinmehall/nusb), a pure rust library similar to the very popular libusb.

For WASM, this crate utilizes [web-sys](https://crates.io/crates/web-sys) which gives access to browser API calls, and in this case is used to interact with [WebUSB](https://developer.mozilla.org/en-US/docs/Web/API/WebUSB_API)

## TODO

- [ ] Add choice for native backend between `libusb` wrapper and pure rust `nusb`
- [ ] Allow platform-specific operations if the user requires them
- [ ] Hot plug support... requires either using `libusb` as an optional backend or for [`nusb` to implement it](https://github.com/kevinmehall/nusb/issues/5)