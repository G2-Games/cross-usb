#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use std::error::Error;
use wasm_bindgen::prelude::*;

use web_sys::{
    console,
    Usb,
    UsbDevice as WasmUsbDevice,
    UsbInterface as WasmUsbInterface,
    UsbControlTransferParameters,
    UsbInTransferResult,
    UsbOutTransferResult,
    UsbRecipient,
    UsbRequestType,
    UsbDeviceRequestOptions,
};
use js_sys::{Array, Uint8Array, Promise, Object};
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
        Ok(dev) => dev.into(),
        Err(err) => {
            console::log_1(&err.clone());
            return Err(err.into())
        },
    };

    let _open_promise = JsFuture::from(Promise::resolve(&device.open())).await?;

    Ok(UsbDevice {
        device
    })
}

impl Device for UsbDevice {
    type UsbDevice = UsbDevice;
    type UsbInterface = UsbInterface;

    async fn open_interface(&self, number: u8) -> Result<UsbInterface, Box<dyn Error>> {
        let dev_promise = JsFuture::from(Promise::resolve(&self.device.claim_interface(number))).await;

        // Wait for the interface to be claimed
        let _device: WasmUsbDevice = match dev_promise {
            Ok(dev) => dev.into(),
            Err(err) => {
                console::log_1(&err.clone());
                return Err(format!("{:?}", err).into())
            },
        };

        Ok(UsbInterface {
            device: self.device.clone()
        })
    }

    async fn reset(&self) -> Result<(), Box<dyn Error>> {
        let promise = Promise::resolve(&self.device.reset());

        let result = JsFuture::from(promise).await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("cancelled".into()),
        }
    }

    async fn vendor_id(&self) -> u16 {
        self.device.vendor_id()
    }

    async fn product_id(&self) -> u16 {
        self.device.product_id()
    }

    async fn class(&self) -> u8 {
        self.device.device_class()
    }

    async fn subclass(&self) -> u8 {
        self.device.device_subclass()
    }

    async fn manufacturer_string(&self) -> Option<String> {
        self.device.manufacturer_name()
    }

    async fn product_string(&self) -> Option<String> {
        self.device.product_name()
    }
}

impl<'a> Interface<'a> for UsbInterface {
    async fn control_in(&self, data: crate::usb::ControlIn) -> Result<Vec<u8>, Box<dyn Error>> {
        let length = data.length;
        let params: UsbControlTransferParameters = data.into();

        let promise = Promise::resolve(&self.device.control_transfer_in(&params, length));
        let result = JsFuture::from(promise).await;

        let transfer_result: UsbInTransferResult = match result {
            Ok(res) => res.into(),
            Err(err) => return Err(format!("Error {:?}", err).into()),
        };

        let data = match transfer_result.data() {
            Some(res) => res.buffer(),
            None => return Err("No data returned".into()),
        };

        let array = Uint8Array::new(&data);

        Ok(array.to_vec())
    }

    async fn control_out(&self, data: crate::usb::ControlOut<'a>) -> Result<(), Box<dyn Error>> {
        let array = Uint8Array::from(data.data);
        let array_obj = Object::try_from(&array).unwrap();
        let params: UsbControlTransferParameters = data.into();

        let promise = Promise::resolve(&self.device.control_transfer_out_with_buffer_source(&params, array_obj));
        let result = JsFuture::from(promise).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("{:?}", err).into()),
        }
    }

    async fn bulk_in(&self, endpoint: u8, length: usize) -> Result<Vec<u8>, Box<dyn Error>> {
        let promise = Promise::resolve(&self.device.transfer_in(endpoint, length as u32));

        let result = JsFuture::from(promise).await;

        let transfer_result: UsbInTransferResult = match result {
            Ok(res) => res.into(),
            Err(err) => return Err(format!("Error {:?}", err).into()),
        };

        let data = match transfer_result.data() {
            Some(res) => res.buffer(),
            None => return Err("No data returned".into()),
        };

        let array = Uint8Array::new(&data);

        Ok(array.to_vec())
    }

    async fn bulk_out(&self, endpoint: u8, data: &[u8]) -> Result<usize, Box<dyn Error>> {
        let array = Uint8Array::from(data);
        let array_obj = Object::try_from(&array).unwrap();

        let promise = Promise::resolve(&self.device.transfer_out_with_buffer_source(endpoint, array_obj));

        let result = JsFuture::from(promise).await;

        let transfer_result: UsbOutTransferResult = match result {
            Ok(res) => res.into(),
            Err(err) => return Err(format!("Error {:?}", err).into()),
        };

        Ok(transfer_result.bytes_written() as usize)
    }
}

impl From<ControlIn> for UsbControlTransferParameters {
    fn from(value: ControlIn) -> Self {
        UsbControlTransferParameters::new(
            value.index,
            value.recipient.into(),
            value.request,
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
            value.request,
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
