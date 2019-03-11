use crate::Device;
use crate::pixels::{DataBits};
use hal;

pub trait OutputRows {
    const WIDTH: usize;
    const HEIGHT: usize;

    fn output_pixels<I2C, DATA>(&self, device: &mut Device<I2C>, data: &DATA) -> Result<(), I2C::Error>
        where
            I2C: hal::blocking::i2c::Write,
            DATA: DataBits
    {
        let mut buffer: [u8; 11] = [0; 11];

        for row_index in 0..Self::HEIGHT {
            for byte in data.row_bytes(row_index as i16, 0..1) {
                buffer[row_index] = byte;
            }
        }

        self.write_buffer(device, &buffer[0..Self::HEIGHT])
    }

    fn write_buffer<I2C>(&self, device: &mut Device<I2C>, buffer: &[u8]) -> Result<(), I2C::Error>
        where
            I2C: hal::blocking::i2c::Write;
}

pub struct MatrixTargetPrimary8x8 {

}

impl OutputRows for MatrixTargetPrimary8x8 {
    const WIDTH: usize = 8;
    const HEIGHT: usize = 8;

    fn write_buffer<I2C>(&self, device: &mut Device<I2C>, buffer: &[u8]) -> Result<(), I2C::Error>
        where
            I2C: hal::blocking::i2c::Write
    {
        device.set_matrix1_rows(0, buffer)
    }
}

pub struct MatrixTargetSecondary8x8 {

}

impl OutputRows for MatrixTargetSecondary8x8 {
    const WIDTH: usize = 8;
    const HEIGHT: usize = 8;

    fn write_buffer<I2C>(&self, device: &mut Device<I2C>, buffer: &[u8]) -> Result<(), I2C::Error>
        where
            I2C: hal::blocking::i2c::Write
    {
        device.set_matrix2_rows(0, buffer)
    }
}

struct MatrixTargetPrimary7x9 {

}

struct MatrixTargetSecondary7x9 {

}

