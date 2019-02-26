#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Register {
    /// Configuration Register, default 0000 0000
    Config = 0x00,
    /// Lighting Effect Register, Store the intensity control settings, default 0000 0000
    LightingEffect = 0x0d,
    /// Matrix 1 Data Register, Store the on or off state of each LED, default 0000 0000
    Matrix1Begin = 0x01,
    Matrix1End = 0x0b,
    /// Matrix 2 Data Register, Store the on or off state of each LED, default 0000 0000
    Matrix2Begin = 0x0e,
    Matrix2End = 0x18,
    /// Update Column Register
    UpdateColumn = 0x0c,
    /// PWM Register, Modulate LED light with 128 different items, default 1000 0000
    Pwm = 0x19,
    /// Reset Register, Reset all registers to default value
    Reset = 0xff,
}