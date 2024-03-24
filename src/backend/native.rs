use crate::usb::{
    ControlIn, ControlOut, ControlType, UsbDescriptor, UsbDevice, UsbInterface, Recipient, UsbError,
};

#[derive(Clone, Debug)]
pub struct Descriptor {
    device_info: nusb::DeviceInfo,
}

#[derive(Clone)]
pub struct Device {
    device_info: Descriptor,
    device: nusb::Device,
}

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.device_info)
    }
}

#[derive(Clone)]
pub struct Interface {
    interface: nusb::Interface,
    number: u8,
}

impl std::fmt::Debug for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interface {:?}", self.number)
    }
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

pub async fn get_device(
    device_filters: Vec<DeviceFilter>
) -> Result<Descriptor, UsbError> {
    let devices = nusb::list_devices().unwrap();

    let mut device_info = None;
    for prelim_dev_inf in devices {
        // See if the device exists in the list
        if device_filters.iter().any(|info| {
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
        }) {
            device_info = Some(prelim_dev_inf);
            break;
        }
    }

    let device_info = match device_info {
        Some(dev) => dev,
        None => return Err(UsbError::DeviceNotFound),
    };

    Ok(Descriptor { device_info })
}

pub async fn get_device_list(
    device_filters: Vec<DeviceFilter>,
) -> Result<impl Iterator<Item = Descriptor>, UsbError> {
    let devices_info = nusb::list_devices().unwrap();

    let mut devices = Vec::new();
    for prelim_dev_inf in devices_info {
        // See if the device exists in the list
        if device_filters.iter().any(|info| {
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
        }) {
            devices.push(prelim_dev_inf);
        }
    }

    if devices.is_empty() {
        return Err(UsbError::DeviceNotFound);
    }

    let devices_opened: Vec<Descriptor> = devices
        .into_iter()
        .map(|d| Descriptor { device_info: d })
        .collect();

    Ok(devices_opened.into_iter())
}

impl UsbDescriptor for Descriptor {
    type Device = Device;

    async fn open(self) -> Result<Self::Device, UsbError> {
        match self.device_info.open() {
            Ok(dev) => Ok(Self::Device {
                device_info: self,
                device: dev,
            }),
            Err(err) => Err(UsbError::CommunicationError(err.to_string())),
        }
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

impl UsbDevice for Device {
    type Interface = Interface;

    async fn open_interface(&self, number: u8) -> Result<Self::Interface, UsbError> {
        let interface = match self.device.claim_interface(number) {
            Ok(inter) => inter,
            Err(err) => return Err(UsbError::CommunicationError(err.to_string())),
        };

        Ok(Interface {
            interface,
            number
        })
    }

    async fn detach_and_open_interface(&self, number: u8) -> Result<Self::Interface, UsbError> {
        let interface = match self.device.detach_and_claim_interface(number) {
            Ok(inter) => inter,
            Err(err) => return Err(UsbError::CommunicationError(err.to_string())),
        };

        Ok(Interface {
            interface,
            number
        })
    }

    async fn reset(&self) -> Result<(), UsbError> {
        match self.device.reset() {
            Ok(_) => Ok(()),
            Err(err) => Err(UsbError::CommunicationError(err.to_string())),
        }
    }

    async fn forget(&self) -> Result<(), UsbError> {
        self.reset().await
    }

    async fn product_id(&self) -> u16 {
        self.device_info.product_id().await
    }

    async fn vendor_id(&self) -> u16 {
        self.device_info.vendor_id().await
    }

    async fn class(&self) -> u8 {
        self.device_info.class().await
    }

    async fn subclass(&self) -> u8 {
        self.device_info.subclass().await
    }

    async fn manufacturer_string(&self) -> Option<String> {
        self.device_info.manufacturer_string().await
    }

    async fn product_string(&self) -> Option<String> {
        self.device_info.product_string().await
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        let _ = self.device.reset();
    }
}

impl<'a> UsbInterface<'a> for Interface {
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
