#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Address {
    /// AD connected to GND.
    ///
    /// 11000 00
    Address00 = 0b11000_00,
    /// AD connected to VCC.
    ///
    /// 11000 11
    Address11 = 0b11000_11,
    /// AD connected to SCL.
    ///
    /// 11000 01
    Address01 = 0b11000_01,
    /// AD connected to SDA.
    ///
    /// 11000 10
    Address10 = 0b11000_10,
}