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
> When compiling this crate on a WASM target, you must use either
> `RUSTFLAGS=--cfg=web_sys_unstable_apis` or by passing the argument in a
> `.cargo/config.toml` file. Read more here: https://rustwasm.github.io/wasm-bindgen/web-sys/unstable-apis.html

## Dependencies

For native USB, the crate utilizies [nusb](https://github.com/kevinmehall/nusb), a pure rust library similar to the very popular libusb.

For WASM, this crate utilizes [web-sys](https://crates.io/crates/web-sys) which gives access to browser API calls, and in this case is used to interact with [WebUSB](https://developer.mozilla.org/en-US/docs/Web/API/WebUSB_API)

## TODO

- [ ] Add choice for native backend between `libusb` wrapper and pure rust `nusb`
- [ ] Allow platform-specific operations if the user requires them
- [ ] Hot plug support... requires either using `libusb` as an optional backend or for [`nusb` to implement it](https://github.com/kevinmehall/nusb/issues/5)
