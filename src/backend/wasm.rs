//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use wasm_bindgen::prelude::*;

use js_sys::{Array, Object, Promise, Uint8Array};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    UsbControlTransferParameters, UsbDevice as WasmUsbDevice, UsbDeviceRequestOptions,
    UsbInTransferResult, UsbOutTransferResult, UsbRecipient, UsbRequestType,
};

// Crate stuff
use crate::usb::{
    ControlIn, ControlOut, ControlType, UsbDeviceInfo, UsbDevice, UsbInterface, Recipient, Error,
};

#[wasm_bindgen]
#[derive(Debug)]
pub struct DeviceInfo {
    device: WasmUsbDevice,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Device {
    device: WasmUsbDevice,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Interface {
    device: WasmUsbDevice,
    _number: u8,
}

#[wasm_bindgen]
#[derive(PartialEq, Clone, Default)]
pub struct DeviceFilter {
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
    pub class: Option<u8>,
    pub subclass: Option<u8>,
    pub protocol: Option<u8>,
}

impl DeviceFilter {
    pub fn new(
        vendor_id: Option<u16>,
        product_id: Option<u16>,
        class: Option<u8>,
        subclass: Option<u8>,
        protocol: Option<u8>,
    ) -> Self {
        Self {
            vendor_id,
            product_id,
            class,
            subclass,
            protocol,
        }
    }
}

#[wasm_bindgen]
pub async fn get_device(device_filter: Vec<DeviceFilter>) -> Result<DeviceInfo, js_sys::Error> {
    let window = web_sys::window().unwrap();

    let navigator = window.navigator();
    let usb = navigator.usb();

    let device_list: Array = match JsFuture::from(Promise::resolve(&usb.get_devices())).await {
        Ok(list) => list.into(),
        Err(_) => Array::new(),
    };

    // Check if the device is already paired, if so, we don't need to request it again
    for js_device in device_list {
        let device: WasmUsbDevice = js_device.into();

        if device_filter.iter().any(|info| {
            let mut result = false;

            if info.vendor_id.is_some() {
                result = info.vendor_id.unwrap() == device.vendor_id();
            }

            if info.product_id.is_some() {
                result = info.product_id.unwrap() == device.product_id();
            }

            if info.class.is_some() {
                result = info.class.unwrap() == device.device_class();
            }

            if info.subclass.is_some() {
                result = info.subclass.unwrap() == device.device_subclass();
            }

            if info.protocol.is_some() {
                result = info.protocol.unwrap() == device.device_protocol();
            }

            result
        }) {
            let _open_promise = JsFuture::from(Promise::resolve(&device.open())).await?;
            return Ok(DeviceInfo { device });
        }
    }

    let arr = Array::new();
    for filter in device_filter {
        let js_filter = js_sys::Object::new();
        if let Some(vid) = filter.vendor_id {
            js_sys::Reflect::set(
                &js_filter,
                &JsValue::from_str("vendorId"),
                &JsValue::from(vid),
            )
            .unwrap();
        }
        if let Some(pid) = filter.product_id {
            js_sys::Reflect::set(
                &js_filter,
                &JsValue::from_str("productId"),
                &JsValue::from(pid),
            )
            .unwrap();
        }
        if let Some(class) = filter.class {
            js_sys::Reflect::set(
                &js_filter,
                &JsValue::from_str("classCode"),
                &JsValue::from(class),
            )
            .unwrap();
        }
        if let Some(subclass) = filter.subclass {
            js_sys::Reflect::set(
                &js_filter,
                &JsValue::from_str("subclassCode"),
                &JsValue::from(subclass),
            )
            .unwrap();
        }
        if let Some(pro) = filter.protocol {
            js_sys::Reflect::set(
                &js_filter,
                &JsValue::from_str("protocolCode"),
                &JsValue::from(pro),
            )
            .unwrap();
        }
        arr.push(&js_filter);
    }

    let filters = JsValue::from(&arr);
    let filters2 = UsbDeviceRequestOptions::new(&filters);

    let device: WasmUsbDevice = JsFuture::from(Promise::resolve(&usb.request_device(&filters2)))
        .await?
        .into();

    let _open_promise = JsFuture::from(Promise::resolve(&device.open())).await?;

    Ok(DeviceInfo { device })
}

#[wasm_bindgen]
pub async fn get_device_list(device_filter: Vec<DeviceFilter>) -> Result<Vec<DeviceInfo>, js_sys::Error> {
    let window = web_sys::window().unwrap();

    let navigator = window.navigator();
    let usb = navigator.usb();

    let device_list: Array = match JsFuture::from(Promise::resolve(&usb.get_devices())).await {
        Ok(list) => list.into(),
        Err(_) => Array::new(),
    };

    let mut devices = Vec::new();
    // Check if the device is already paired, if so, we don't need to request it again
    for js_device in device_list {
        let device: WasmUsbDevice = js_device.into();

        if device_filter.iter().any(|info| {
            let mut result = false;

            if info.vendor_id.is_some() {
                result = info.vendor_id.unwrap() == device.vendor_id();
            }

            if info.product_id.is_some() {
                result = info.product_id.unwrap() == device.product_id();
            }

            if info.class.is_some() {
                result = info.class.unwrap() == device.device_class();
            }

            if info.subclass.is_some() {
                result = info.subclass.unwrap() == device.device_subclass();
            }

            if info.protocol.is_some() {
                result = info.protocol.unwrap() == device.device_protocol();
            }

            result
        }) {
            let _open_promise = JsFuture::from(Promise::resolve(&device.open())).await?;
            devices.push(DeviceInfo { device });
        }
    }

    let arr = Array::new();
    for filter in device_filter {
        let js_filter = js_sys::Object::new();
        if let Some(vid) = filter.vendor_id {
            js_sys::Reflect::set(
                &js_filter,
                &JsValue::from_str("vendorId"),
                &JsValue::from(vid),
            )
            .unwrap();
        }
        if let Some(pid) = filter.product_id {
            js_sys::Reflect::set(
                &js_filter,
                &JsValue::from_str("productId"),
                &JsValue::from(pid),
            )
            .unwrap();
        }
        if let Some(class) = filter.class {
            js_sys::Reflect::set(
                &js_filter,
                &JsValue::from_str("classCode"),
                &JsValue::from(class),
            )
            .unwrap();
        }
        if let Some(subclass) = filter.subclass {
            js_sys::Reflect::set(
                &js_filter,
                &JsValue::from_str("subclassCode"),
                &JsValue::from(subclass),
            )
            .unwrap();
        }
        if let Some(pro) = filter.protocol {
            js_sys::Reflect::set(
                &js_filter,
                &JsValue::from_str("protocolCode"),
                &JsValue::from(pro),
            )
            .unwrap();
        }
        arr.push(&js_filter);
    }

    let filters = JsValue::from(&arr);
    let filters2 = UsbDeviceRequestOptions::new(&filters);

    let device: WasmUsbDevice = JsFuture::from(Promise::resolve(&usb.request_device(&filters2)))
        .await?
        .into();

    let _open_promise = JsFuture::from(Promise::resolve(&device.open())).await?;

    devices.push(DeviceInfo { device });

    return Ok(devices);
}

impl UsbDeviceInfo for DeviceInfo {
    type Device = Device;

    async fn open(self) -> Result<Self::Device, Error> {
        Ok(Self::Device {
            device: self.device,
        })
    }

    async fn product_id(&self) -> u16 {
        self.device.product_id()
    }

    async fn vendor_id(&self) -> u16 {
        self.device.vendor_id()
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

impl UsbDevice for Device {
    type Interface = Interface;

    async fn open_interface(&self, number: u8) -> Result<Interface, Error> {
        let dev_promise =
            JsFuture::from(Promise::resolve(&self.device.claim_interface(number))).await;

        // Wait for the interface to be claimed
        let _device: WasmUsbDevice = match dev_promise {
            Ok(dev) => dev.into(),
            Err(err) => {
                return Err(Error::CommunicationError(
                    err.as_string().unwrap_or_default(),
                ));
            }
        };

        Ok(Interface {
            device: self.device.clone(),
            _number: number,
        })
    }

    async fn detach_and_open_interface(&self, number: u8) -> Result<Self::Interface, Error> {
        self.open_interface(number).await
    }

    async fn reset(&self) -> Result<(), Error> {
        let result = JsFuture::from(Promise::resolve(&self.device.reset())).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::CommunicationError(
                err.as_string().unwrap_or_default(),
            )),
        }
    }

    async fn forget(&self) -> Result<(), Error> {
        let result = JsFuture::from(Promise::resolve(&self.device.forget())).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::CommunicationError(
                err.as_string().unwrap_or_default(),
            )),
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

impl<'a> UsbInterface<'a> for Interface {
    async fn control_in(&self, data: crate::usb::ControlIn) -> Result<Vec<u8>, Error> {
        let length = data.length;
        let params: UsbControlTransferParameters = data.into();

        let promise = Promise::resolve(&self.device.control_transfer_in(&params, length));
        let result = JsFuture::from(promise).await;

        let transfer_result: UsbInTransferResult = match result {
            Ok(res) => res.into(),
            Err(_) => return Err(Error::TransferError),
        };

        let data = match transfer_result.data() {
            Some(res) => res.buffer(),
            None => return Err(Error::TransferError),
        };

        let array = Uint8Array::new(&data);

        Ok(array.to_vec())
    }

    async fn control_out(&self, data: crate::usb::ControlOut<'a>) -> Result<usize, Error> {
        let array = Uint8Array::from(data.data);
        let array_obj = Object::try_from(&array).unwrap();
        let params: UsbControlTransferParameters = data.into();

        let result: UsbOutTransferResult = match JsFuture::from(Promise::resolve(
            &self
                .device
                .control_transfer_out_with_buffer_source(&params, array_obj)
                .map_err(|j| Error::CommunicationError(j.as_string().unwrap_or_default()))?
                .into(),
        ))
        .await
        {
            Ok(res) => res.into(),
            Err(_) => return Err(Error::TransferError),
        };

        Ok(result.bytes_written() as usize)
    }

    async fn bulk_in(&self, endpoint: u8, length: usize) -> Result<Vec<u8>, Error> {
        let promise = Promise::resolve(&self.device.transfer_in(endpoint, length as u32));

        let result = JsFuture::from(promise).await;

        let transfer_result: UsbInTransferResult = match result {
            Ok(res) => res.into(),
            Err(_) => return Err(Error::TransferError),
        };

        let data = match transfer_result.data() {
            Some(res) => res.buffer(),
            None => return Err(Error::TransferError),
        };

        let array = Uint8Array::new(&data);

        Ok(array.to_vec())
    }

    async fn bulk_out(&self, endpoint: u8, data: &[u8]) -> Result<usize, Error> {
        let array = Uint8Array::from(data);
        let array_obj = Object::try_from(&array).unwrap();

        let promise = Promise::resolve(
            &self
                .device
                .transfer_out_with_buffer_source(endpoint, array_obj)
                .map_err(|j| Error::CommunicationError(j.as_string().unwrap_or_default()))?
                .into(),
        );

        let result = JsFuture::from(promise).await;

        let transfer_result: UsbOutTransferResult = match result {
            Ok(res) => res.into(),
            Err(_) => return Err(Error::TransferError),
        };

        Ok(transfer_result.bytes_written() as usize)
    }

    /*
    async fn interrupt_in(&self, endpoint: u8, length: usize) -> Result<Vec<u8>, UsbError> {
        let promise = Promise::resolve(&self.device.transfer_in(endpoint, length as u32));

        let result = JsFuture::from(promise).await;

        let transfer_result: UsbInTransferResult = match result {
            Ok(res) => res.into(),
            Err(_) => return Err(UsbError::TransferError),
        };

        if transfer_result.

        let data = match transfer_result.data() {
            Some(res) => res.buffer(),
            None => return Err(UsbError::TransferError),
        };

        let array = Uint8Array::new(&data);

        Ok(array.to_vec())
    }

    async fn interrupt_out(&self, endpoint: u8, buf: Vec<u8>) -> Result<usize, UsbError> {
        todo!()
    }
    */
}

impl From<ControlIn> for UsbControlTransferParameters {
    fn from(value: ControlIn) -> Self {
        UsbControlTransferParameters::new(
            value.index,
            value.recipient.into(),
            value.request,
            value.control_type.into(),
            value.value,
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
            value.value,
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
