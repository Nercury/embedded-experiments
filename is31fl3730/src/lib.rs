#![no_std]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]

extern crate embedded_hal as hal;

mod address;
mod register;
mod lighting;
mod configuration;
pub mod display;
pub mod pixels;

pub use lighting::{
    Lighting,
    LightingAudioGain,
    LightingCurrent,
};
pub use configuration::{
    Configuration,
    ConfigMatrixMode,
    ConfigDisplayMode,
    ConfigAudio,
};
pub use address::{Address};

pub struct Device<I2C>
    where
        I2C: hal::blocking::i2c::Write,
{
    address: Address,
    i2c: I2C,
    lighting: Lighting,
    config: Configuration,
}

impl<I2C, E> Device<I2C>
    where
        I2C: hal::blocking::i2c::Write<Error = E>,
{
    pub fn new(address: Address, i2c: I2C) -> Device<I2C> {
        Device {
            address,
            i2c,
            lighting: Lighting::default(),
            config: Configuration::default(),
        }
    }

    /// Set PWM value (0b0000000 - 0b1111111)
    pub fn set_pwm(&mut self, value: u8) -> Result<(), E> {
        self.i2c.write(self.address as u8, &[register::Register::Pwm as u8, (value & 0b0111_1111) | 0b1000_0000])
    }

    /// Write pixels for the first matrix. Call update to flush updates.
    pub fn set_matrix1_rows(&mut self, start_row: u8, rows: &[u8]) -> Result<(), E> {
        const BUFLEN: usize = 1 + 11;
        let mut writebuf: [u8; BUFLEN] = [0; BUFLEN];

        let final_write_len = rows.len() + 1;

        writebuf[0] = register::Register::Matrix1Begin as u8 + start_row;
        writebuf[1..final_write_len].copy_from_slice(rows);

        self.i2c.write(self.address as u8, &writebuf[..final_write_len])
    }

    /// Write pixels for the second matrix. Call update to flush updates.
    pub fn set_matrix2_rows(&mut self, start_row: u8, rows: &[u8]) -> Result<(), E> {
        const BUFLEN: usize = 1 + 11;
        let mut writebuf: [u8; BUFLEN] = [0; BUFLEN];

        let final_write_len = rows.len() + 1;

        writebuf[0] = register::Register::Matrix2Begin as u8 + start_row;
        writebuf[1..final_write_len].copy_from_slice(rows);

        self.i2c.write(self.address as u8, &writebuf[..final_write_len])
    }

    /// Flush display updates.
    pub fn update(&mut self) -> Result<(), E> {
        self.i2c.write(self.address as u8, &[register::Register::UpdateColumn as u8, 0b00000000])
    }

    /// Reset device.
    pub fn reset(&mut self) -> Result<(), E> {
        self.i2c.write(self.address as u8, &[register::Register::Reset as u8, 0b00000000])
    }

    /// Get lighting configuration.
    pub fn lighting(&self) -> Lighting {
        self.lighting
    }

    /// Modify lighting configuration and send it to device.
    pub fn modify_lighting<F>(&mut self, mut modify: F) -> Result<(), E>
        where F: FnMut(&mut Lighting) -> &mut Lighting {
        let mut lighting = self.lighting;
        modify(&mut lighting);
        self.i2c.write(self.address as u8, &[register::Register::LightingEffect as u8, lighting.byte()])
    }

    /// Get device configuration.
    pub fn config(&self) -> Configuration {
        self.config
    }

    /// Modify device configuration and send it to device.
    pub fn modify_config<F>(&mut self, mut modify: F) -> Result<(), E>
        where F: FnMut(&mut Configuration) -> &mut Configuration {
        let mut config = self.config;
        modify(&mut config);
        self.i2c.write(self.address as u8, &[register::Register::Config as u8, config.byte()])
    }
}