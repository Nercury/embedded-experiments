#![no_std]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]

use heapless::{Vec};
use heapless::ArrayLength;
use bit_field::BitField;

pub mod consts;

#[derive(Copy, Clone)]
pub enum Error {
    WidthCapacityOutOfBounds,
    HeightCapacityOutOfBounds,
    XOutOfBounds,
    YOutOfBounds,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        core::fmt::Display::fmt(match *self {
            Error::WidthCapacityOutOfBounds => "width cap oob",
            Error::HeightCapacityOutOfBounds => "height cap oob",
            Error::XOutOfBounds => "x oob",
            Error::YOutOfBounds => "y oob",
        }, f)
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        core::fmt::Display::fmt(self, f)
    }
}

/// Stack-allocated bit canvas of the specified size.
///
/// Example initialisation and use:
///
/// ```
/// use bitcanvas::{BitCanvas};
/// use bitcanvas::consts::*;
///
/// let canvas = BitCanvas::<W16, H8>::new(15, 5).unwrap();
/// assert_eq!(16, canvas.cap_width());
/// assert_eq!(8, canvas.cap_height());
/// assert_eq!(15, canvas.width());
/// assert_eq!(5, canvas.height());
/// ```
pub struct BitCanvas<W, H>
    where
        W: ArrayLength<u8>,
        H: ArrayLength<Vec<u8, W>>
{
    bit_width: i16,
    height: i16,
    #[cfg(feature = "graphics")]
    _alpha_threshold: u8,
    data: Vec<Vec<u8, W>, H>,
}

impl<W, H> BitCanvas<W, H>
    where
        W: ArrayLength<u8>,
        H: ArrayLength<Vec<u8, W>>
{
    pub fn new(width: i16, height: i16) -> Result<BitCanvas<W, H>, Error> {
        let mut byte_width = width / 8;
        if width % 8 > 0 {
            byte_width += 1;
        }

        if byte_width > W::I16 {
            return Err(Error::WidthCapacityOutOfBounds);
        }

        if height > H::I16 {
            return Err(Error::HeightCapacityOutOfBounds);
        }

        let mut row_data = Vec::new();
        row_data.extend(::core::iter::repeat(0).take(byte_width as usize));

        let mut data = Vec::new();
        data.extend(::core::iter::repeat(row_data).take(height as usize));

        Ok(BitCanvas {
            bit_width: width,
            height,
            data,
            #[cfg(feature = "graphics")]
            _alpha_threshold: 0,
        })
    }

    /// The height capacity in pixels
    pub fn cap_height(&self) -> i16 {
        H::I16
    }

    /// The width capacity in pixels
    pub fn cap_width(&self) -> i16 {
        W::I16 * 8
    }

    pub fn width(&self) -> i16 {
        self.bit_width
    }

    pub fn height(&self) -> i16 {
        self.height
    }

    /// Retrieve row at index
    pub fn row(&self, y: i16) -> Result<&[u8], Error> {
        if y < 0 { return Err(Error::YOutOfBounds) }
        Ok(&self.data.get(y as usize).ok_or(Error::YOutOfBounds)?)
    }

    /// Read whole byte at specified location.
    ///
    /// Let's say we initialize matrix with this data:
    ///
    /// ```text
    /// 0000 0000
    /// 0000 1100
    /// ```
    ///
    /// ```
    /// use bitcanvas::{BitCanvas, BitCanvasView};
    /// use bitcanvas::consts::*;
    ///
    /// // Initialize
    ///
    /// let mut canvas = BitCanvas::<W8, H8>::new(8, 8).unwrap();
    /// canvas.set_bit(4, 1, true);
    /// canvas.set_bit(5, 1, true);
    ///
    /// // Read first (0) byte at second (1) row:
    ///
    /// assert_eq!(0b0000_1100, canvas.byte(0, 1).unwrap());
    /// ```
    pub fn byte(&mut self, byte_x: i16, y: i16) -> Result<u8, Error> {
        let row = self.row(y)?;
        if byte_x < 0 || byte_x as usize >= row.len() { return Err(Error::XOutOfBounds) }
        Ok(*unsafe { row.get_unchecked(byte_x as usize) })
    }

    /// Get the byte and bit offset at specified (x, y) location.
    pub fn byte_and_bit(&mut self, x: i16, y: i16) -> Result<(u8, u8), Error> {
        Ok((self.byte(x / 8, y)?, 7 - (x % 8) as u8))
    }

    /// Get bit at specified (x, y) location
    pub fn bit(&mut self, x: i16, y: i16) -> Result<bool, Error> {
        let (byte, bit) = self.byte_and_bit(x, y)?;
        let val = byte.get_bit(bit as usize);

        Ok(val)
    }

    /// Retrieve mutable row at index
    pub fn row_mut(&mut self, y: i16) -> Result<&mut Vec<u8, W>, Error> {
        if y < 0 { return Err(Error::YOutOfBounds) }
        self.data.get_mut(y as usize).ok_or(Error::YOutOfBounds)
    }

    /// Get a pointer to 8 bits as byte at specified location.
    ///
    /// Example: modify the 8 bytes at the lower right corner as byte.
    ///
    /// ```
    /// use bitcanvas::BitCanvas;
    /// use bitcanvas::consts::*;
    ///
    /// let mut canvas = BitCanvas::<W16, H8>::new(15, 2).unwrap();
    /// *canvas.byte_mut(1, 1).unwrap() = 0b0101_1010;
    /// ```
    ///
    /// The code above modifies the data to look like this:
    ///
    /// ```text
    /// 0000 0000 0000 000
    /// 0000 0000 0101 101
    /// ```
    ///
    /// The x index is for byte, not for bit, and can overwrite bits up to defined capacity.
    pub fn byte_mut(&mut self, byte_x: i16, y: i16) -> Result<&mut u8, Error> {
        let row = self.row_mut(y)?;
        if byte_x < 0 { return Err(Error::XOutOfBounds) }
        row.get_mut(byte_x as usize).ok_or(Error::XOutOfBounds)
    }

    /// Get a mutable pointer to byte and the bit offset at specified (x, y) location.
    pub fn byte_and_bit_mut(&mut self, x: i16, y: i16) -> Result<(&mut u8, u8), Error> {
        Ok((self.byte_mut(x / 8, y)?, 7 - (x % 8) as u8))
    }

    /// Flip bit at specified (x, y) location.
    pub fn set_bit(&mut self, x: i16, y: i16, value: bool) -> Result<(), Error> {
        let (byte, bit) = self.byte_and_bit_mut(x, y)?;
        byte.set_bit(bit as usize, value);

        Ok(())
    }

    /// Get the alpha threshold used for conversion of alpha value to monochrome bit.
    #[cfg(feature = "graphics")]
    pub fn alpha_threshold(&self) -> u8 {
        self._alpha_threshold
    }

    /// Set the threshold value for converting 8-bit alpha value to monochrome bit.
    ///
    /// The value will be converted to "true" if it is above this threshold.
    #[cfg(feature = "graphics")]
    pub fn set_alpha_threshold(&mut self, value: u8) {
        self._alpha_threshold = value;
    }
}

#[cfg(feature = "graphics")]
use embedded_graphics::{drawable, pixelcolor::PixelColorU8, Drawing};

#[cfg(feature = "graphics")]
impl<W, H> Drawing<PixelColorU8> for BitCanvas<W, H>
    where
        W: ArrayLength<u8>,
        H: ArrayLength<Vec<u8, W>>
{
    fn draw<T>(&mut self, item_pixels: T)
        where
            T: Iterator<Item = drawable::Pixel<PixelColorU8>>,
    {
        for pixel in item_pixels {
            let _ = self.set_bit((pixel.0).0 as i16, (pixel.0).1 as i16, pixel.1.into_inner() > self._alpha_threshold);
        }
    }
}