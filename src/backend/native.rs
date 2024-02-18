use crate::usb::{ControlIn, ControlOut, ControlType, Device, Interface, Recipient, UsbError};

pub struct UsbDevice {
    device_info: nusb::DeviceInfo,
    device: nusb::Device,
}

pub struct UsbInterface {
    interface: nusb::Interface,
}

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

pub async fn get_device(device_filter: Vec<DeviceFilter>) -> Result<UsbDevice, UsbError> {
    let devices = nusb::list_devices().unwrap();

    let mut device_info = None;
    for prelim_dev_inf in devices {
        // See if the device exists in the list
        if device_filter
            .iter()
            .any(|info| {
                let mut result = false;

                if info.vendor_id.is_some() {
                    result = info.vendor_id.unwrap() == prelim_dev_inf.vendor_id();
                }

                if info.product_id.is_some() {
                    result = info.product_id.unwrap() == prelim_dev_inf.product_id();
                }

                if info.class.is_some() {
                    result = info.class.unwrap() == prelim_dev_inf.class();
                }

                if info.subclass.is_some() {
                    result = info.subclass.unwrap() == prelim_dev_inf.subclass();
                }

                if info.protocol.is_some() {
                    result = info.protocol.unwrap() == prelim_dev_inf.protocol();
                }

                result
            })
        {
            device_info = Some(prelim_dev_inf);
            break
        }
    }

    let device_info = match device_info {
        Some(dev) => dev,
        None => return Err(UsbError::DeviceNotFound),
    };

    let device = match device_info.open() {
        Ok(dev) => dev,
        Err(_) => return Err(UsbError::CommunicationError),
    };

    Ok(UsbDevice {
        device_info,
        device,
    })
}

impl Device for UsbDevice {
    type UsbDevice = UsbDevice;
    type UsbInterface = UsbInterface;

    async fn open_interface(&self, number: u8) -> Result<UsbInterface, UsbError> {
        let interface = match self.device.claim_interface(number) {
            Ok(inter) => inter,
            Err(_) => return Err(UsbError::CommunicationError),
        };

        Ok(UsbInterface { interface })
    }

    async fn reset(&self) -> Result<(), UsbError> {
        match self.device.reset() {
            Ok(_) => Ok(()),
            Err(_) => Err(UsbError::CommunicationError),
        }
    }

    async fn forget(&self) -> Result<(), UsbError> {
        self.reset().await
    }

    async fn vendor_id(&self) -> u16 {
        self.device_info.vendor_id()
    }

    async fn product_id(&self) -> u16 {
        self.device_info.product_id()
    }

    async fn class(&self) -> u8 {
        self.device_info.class()
    }

    async fn subclass(&self) -> u8 {
        self.device_info.subclass()
    }

    async fn manufacturer_string(&self) -> Option<String> {
        self.device_info.manufacturer_string().map(str::to_string)
    }

    async fn product_string(&self) -> Option<String> {
        self.device_info.product_string().map(str::to_string)
    }
}

impl<'a> Interface<'a> for UsbInterface {
    async fn control_in(&self, data: ControlIn) -> Result<Vec<u8>, UsbError> {
        let result = match self.interface.control_in(data.into()).await.into_result() {
            Ok(res) => res,
            Err(_) => return Err(UsbError::TransferError),
        };

        Ok(result)
    }

    async fn control_out(&self, data: ControlOut<'a>) -> Result<usize, UsbError> {
        match self.interface.control_out(data.into()).await.into_result() {
            Ok(bytes) => Ok(bytes.actual_length()),
            Err(_) => Err(UsbError::TransferError),
        }
    }

    async fn bulk_in(&self, endpoint: u8, length: usize) -> Result<Vec<u8>, UsbError> {
        let request_buffer = nusb::transfer::RequestBuffer::new(length);

        match self
            .interface
            .bulk_in(endpoint, request_buffer)
            .await
            .into_result()
        {
            Ok(res) => Ok(res),
            Err(_) => Err(UsbError::TransferError),
        }
    }

    async fn bulk_out(&self, endpoint: u8, data: &[u8]) -> Result<usize, UsbError> {
        match self
            .interface
            .bulk_out(endpoint, data.to_vec())
            .await
            .into_result()
        {
            Ok(len) => Ok(len.actual_length()),
            Err(_) => Err(UsbError::TransferError),
        }
    }

    /*
    async fn interrupt_in(&self, endpoint: u8, length: usize) -> Result<Vec<u8>, UsbError> {
        let buf = Vec::new();
        let buffer = nusb::transfer::RequestBuffer::reuse(buf, length);

        match self.interface.interrupt_in(endpoint, buffer).await.into_result() {
            Ok(res) => Ok(res),
            Err(_) => Err(UsbError::TransferError),
        }
    }

    async fn interrupt_out(&self, endpoint: u8, buf: Vec<u8>) -> Result<usize, UsbError> {
        match self.interface.interrupt_out(endpoint, buf).await.into_result() {
            Ok(res) => Ok(res.actual_length()),
            Err(_) => Err(UsbError::TransferError),
        }
    }
    */
}

impl From<ControlIn> for nusb::transfer::ControlIn {
    fn from(val: ControlIn) -> Self {
        nusb::transfer::ControlIn {
            control_type: val.control_type.into(),
            recipient: val.recipient.into(),
            request: val.request,
            value: val.value,
            index: val.index,
            length: val.length,
        }
    }
}

impl<'a> From<ControlOut<'a>> for nusb::transfer::ControlOut<'a> {
    fn from(val: ControlOut<'a>) -> Self {
        nusb::transfer::ControlOut {
            control_type: val.control_type.into(),
            recipient: val.recipient.into(),
            request: val.request,
            value: val.value,
            index: val.index,
            data: val.data,
        }
    }
}

impl From<ControlType> for nusb::transfer::ControlType {
    fn from(val: ControlType) -> Self {
        match val {
            ControlType::Standard => nusb::transfer::ControlType::Standard,
            ControlType::Class => nusb::transfer::ControlType::Class,
            ControlType::Vendor => nusb::transfer::ControlType::Vendor,
        }
    }
}

impl From<Recipient> for nusb::transfer::Recipient {
    fn from(val: Recipient) -> Self {
        match val {
            Recipient::Device => nusb::transfer::Recipient::Device,
            Recipient::Interface => nusb::transfer::Recipient::Interface,
            Recipient::Endpoint => nusb::transfer::Recipient::Endpoint,
            Recipient::Other => nusb::transfer::Recipient::Other,
        }
    }
}
