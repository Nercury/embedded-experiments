#![no_std]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]

extern crate embedded_hal as hal;

mod address;
pub mod config;

pub use config::Configuration;
use core::marker::PhantomData;
pub use address::{DeviceAddress, Address00, Address01, Address10, Address11};

pub struct Device<A, I2C>
    where
        A: address::DeviceAddress,
        I2C: hal::blocking::i2c::Write,
{
    _address: PhantomData<A>,
    i2c: I2C,
    config: Configuration,
}

impl<A, I2C> Device<A, I2C>
    where
        A: address::DeviceAddress,
        I2C: hal::blocking::i2c::Write,
{
    pub fn with_default_config(i2c: I2C) -> Device<A, I2C> {
        Device {
            _address: PhantomData::<A>,
            i2c,
            config: Configuration::default(),
        }
    }

    pub fn set_current(&mut self, current: config::Current) -> Result<(), ()> {
        self.i2c.write(A::DEVICE_ADDRESS, &[config::Register::LightingEffect as u8, current as u8])
            .map_err(|_| ())
    }

    pub fn set_matrix1_columns_rows(&mut self, start_column: u8, rows: &[u8]) -> Result<(), ()> {
        const BUFLEN: usize = 1 + 11;
        let mut writebuf: [u8; BUFLEN] = [0; BUFLEN];

        let final_write_len = rows.len() + 1;

        writebuf[0] = config::Register::Matrix1Begin as u8 + start_column;
        writebuf[1..final_write_len].copy_from_slice(rows);

        self.i2c.write(A::DEVICE_ADDRESS, &writebuf[..final_write_len])
            .map_err(|_| ())
    }

    pub fn set_matrix2_columns_rows(&mut self, start_column: u8, rows: &[u8]) -> Result<(), ()> {
        const BUFLEN: usize = 1 + 11;
        let mut writebuf: [u8; BUFLEN] = [0; BUFLEN];

        let final_write_len = rows.len() + 1;

        writebuf[0] = config::Register::Matrix2Begin as u8 + start_column;
        writebuf[1..final_write_len].copy_from_slice(rows);

        self.i2c.write(A::DEVICE_ADDRESS, &writebuf[..final_write_len])
            .map_err(|_| ())
    }

    pub fn update(&mut self) -> Result<(), ()> {
        self.i2c.write(A::DEVICE_ADDRESS, &[config::Register::UpdateColumn as u8, 0b00000000])
            .map_err(|_| ())
    }

    pub fn modify_config<F>(&mut self, mut modify: F) -> Result<(), ()>
        where F: FnMut(&mut Configuration) -> &mut Configuration {
        let mut config = self.config;
        modify(&mut config);
        self.i2c.write(A::DEVICE_ADDRESS, &[config::Register::Config as u8, config.byte()])
            .map_err(|_| ())
    }
}