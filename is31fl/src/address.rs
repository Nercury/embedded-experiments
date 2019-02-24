/// AD connected to GND.
///
/// 11000 00
pub enum Address00 {}
impl DeviceAddress for Address00 {
    const DEVICE_ADDRESS: u8 = 0b11000_00;
}

/// AD connected to VCC.
///
/// 11000 11
pub enum Address11 {}
impl DeviceAddress for Address11 {
    const DEVICE_ADDRESS: u8 = 0b11000_11;
}

/// AD connected to SCL.
///
/// 11000 01
pub enum Address01 {}
impl DeviceAddress for Address01 {
    const DEVICE_ADDRESS: u8 = 0b11000_01;
}

/// AD connected to SDA.
///
/// 11000 10
pub enum Address10 {}
impl DeviceAddress for Address10 {
    const DEVICE_ADDRESS: u8 = 0b11000_10;
}

pub trait DeviceAddress {
    const DEVICE_ADDRESS: u8;
}