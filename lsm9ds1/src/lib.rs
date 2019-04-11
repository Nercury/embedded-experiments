#![no_std]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]

extern crate embedded_hal as hal;

mod register {
    #[derive(Copy, Clone)]
    #[repr(u8)]
    pub enum Register {
        /// Configuration Register, default 0000 0000
        Config = 0x00,
    }
}

pub struct Device<I2C>
    where
        I2C: hal::blocking::i2c::Write,
{
    acc_gyro_address: u8,
    i2c: I2C,
}

impl<I2C, E> Device<I2C>
    where
        I2C: hal::blocking::i2c::Write<Error = E>,
{
    pub fn new(i2c: I2C) -> Device<I2C> {
        Device {
            acc_gyro_address: 0b110101_0, // last bit is external
            i2c,
        }
    }

    pub fn set_gyro_on(&mut self, value: bool) -> Result<(), E> {
        self.i2c.write(self.acc_gyro_address as u8, &[register::Register::Pwm as u8, (value & 0b0111_1111) | 0b1000_0000])
    }
}