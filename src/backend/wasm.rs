#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use std::error::Error;
use wasm_bindgen::prelude::*;
use js_sys::JSON;

use web_sys::{
    console,
    Usb,
    UsbDevice as WasmUsbDevice,
    UsbInterface as WasmUsbInterface,
    UsbControlTransferParameters,
    UsbRecipient,
    UsbRequestType,
    UsbDeviceRequestOptions,
};
use js_sys::{Array, Uint8Array, Promise};
use wasm_bindgen_futures::JsFuture;

// Crate stuff
use crate::usb::{ControlIn, ControlOut, ControlType, Device, Interface, Recipient};

#[wasm_bindgen]
pub struct UsbDevice {
    device: WasmUsbDevice,
}

#[wasm_bindgen]
pub struct UsbInterface {
    device: WasmUsbDevice,
}

/// Gets a single device from the VendorID and ProductID
#[wasm_bindgen]
pub async fn get_device(vendor_id: u16, product_id: u16) -> Result<UsbDevice, js_sys::Error> {
    let window = web_sys::window().unwrap();

    let navigator = window.navigator();
    let usb = navigator.usb();

    let arr = Array::new();
    let filter1 = js_sys::Object::new();
    js_sys::Reflect::set(
        &filter1,
        &JsValue::from_str("vendorId"),
        &JsValue::from(vendor_id),
    )
    .unwrap();
    js_sys::Reflect::set(
        &filter1,
        &JsValue::from_str("productId"),
        &JsValue::from(product_id),
    )
    .unwrap();
    arr.push(&filter1);
    let filters = JsValue::from(&arr);

    let filters2 = UsbDeviceRequestOptions::new(&filters);

    let device_promise = JsFuture::from(Promise::resolve(&usb.request_device(&filters2))).await;

    let device: WasmUsbDevice = match device_promise {
        Ok(res) => res.into(),
        Err(err) => {
            console::log_1(&err.clone().into());
            return Err(err.into())
        },
    };

    let _open_promise = JsFuture::from(Promise::resolve(&device.open()));

    console::log_1(&"got device".into());

    Ok(UsbDevice {
        device
    })
}

impl Device for UsbDevice {
    type UsbDevice = UsbDevice;
    type UsbInterface = UsbInterface;

    async fn open_interface(&self, number: u8) -> Result<UsbInterface, Box<dyn Error>> {
        let dev_promise = JsFuture::from(Promise::resolve(&self.device.claim_interface(number)));

        // Wait for the interface to be claimed
        let result = dev_promise;

        Ok(UsbInterface {
            device: self.device.clone()
        })
    }

    async fn reset(&self) -> Result<(), Box<dyn Error>> {
        let promise = Promise::resolve(&self.device.reset());

        let result = JsFuture::from(promise).await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => {
                console::log_1(&"Cancelled".into());
                return Err("cancelled".into())
            },
        }
    }
}

impl<'a> Interface<'a> for UsbInterface {
    async fn control_in(&self, data: crate::usb::ControlIn) -> Result<Vec<u8>, Box<dyn Error>> {
        let length = data.length;
        let params = data.into();
        let promise = Promise::resolve(&self.device.control_transfer_in(&params, length));

        let mut result = JsFuture::from(promise).await;

        let data = match result {
            Ok(res) => res.into(),
            Err(_) => {
                console::log_1(&"Cancelled".into());
                return Err("cancelled".into())
            },
        };

        let unitarray = Uint8Array::new(&data);

        Ok(unitarray.to_vec())
    }

    async fn control_out(&self, data: crate::usb::ControlOut<'a>) -> Result<(), Box<dyn Error>> {
        let params = data.into();
        let promise = Promise::resolve(&self.device.control_transfer_out(&params));

        let mut result = JsFuture::from(promise).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                console::log_1(&"Cancelled".into());
                Err(format!("{:?}", err).into())
            },
        }
    }

    async fn bulk_in(&self, _endpoint: u8, _buf: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
        todo!()
    }

    async fn bulk_out(&self, _endpoint: u8, _buf: Vec<u8>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl From<ControlIn> for UsbControlTransferParameters {
    fn from(value: ControlIn) -> Self {
        UsbControlTransferParameters::new(
            value.index,
            value.recipient.into(),
            value.request.into(),
            value.control_type.into(),
            value.value
        )
    }
}

impl From<ControlOut<'_>> for UsbControlTransferParameters {
    fn from(value: ControlOut) -> Self {
        UsbControlTransferParameters::new(
            value.index,
            value.recipient.into(),
            value.request.into(),
            value.control_type.into(),
            value.value
        )
    }
}

impl From<Recipient> for UsbRecipient {
    fn from(value: Recipient) -> Self {
        match value {
            Recipient::Device => UsbRecipient::Device,
            Recipient::Interface => UsbRecipient::Interface,
            Recipient::Endpoint => UsbRecipient::Endpoint,
            Recipient::Other => UsbRecipient::Other,
        }
    }
}

impl From<ControlType> for UsbRequestType {
    fn from(value: ControlType) -> Self {
        match value {
            ControlType::Standard => UsbRequestType::Standard,
            ControlType::Class => UsbRequestType::Class,
            ControlType::Vendor => UsbRequestType::Vendor,
        }
    }
}
