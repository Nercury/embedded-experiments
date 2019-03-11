use core::ops::Range;
use bitcanvas::{BitCanvas};
use heapless::{Vec};
use heapless::ArrayLength;

pub trait DataBits {
    type BytesIter: Iterator<Item = u8>;
    type BytesIterRev: Iterator<Item = u8>;

    fn row_bits_len(&self) -> i16;
    fn row_bytes(&self, row: i16, range: Range<i16>) -> Self::BytesIter;
    fn row_bytes_rev(&self, row: i16, range: Range<i16>) -> Self::BytesIterRev;

    fn flip_h(&self) -> FlipH<Self> where Self: Sized {
        FlipH {
            inner: self
        }
    }

    fn offset_bytes(&self, x: i16, y: i16) -> OffsetBytes<Self> where Self: Sized {
        OffsetBytes {
            inner: self,
            offset_x: x,
            offset_y: y,
        }
    }

    fn rotate_90(&self) -> Rotate90<Self> where Self: Sized {
        Rotate90 {
            inner: self,
        }
    }
}

impl<W, H> DataBits for BitCanvas<W, H>
    where
        W: ArrayLength<u8>,
        H: ArrayLength<Vec<u8, W>>
{
    type BytesIter = BytesIter;
    type BytesIterRev = BytesIterRev;

    fn row_bits_len(&self) -> i16 {
        self.width()
    }

    fn row_bytes(&self, row: i16, range: Range<i16>) -> Self::BytesIter {
        if row < 0 {
            BytesIter {
                row: ::core::ptr::null(),
                row_len: 0,
                pos: range.start,
                end: range.end
            }
        } else {
            match self.row(row) {
                Err(_) => BytesIter {
                    row: ::core::ptr::null(),
                    row_len: 0,
                    pos: range.start,
                    end: range.end
                },
                Ok(row) => BytesIter {
                    row: row.as_ptr(),
                    row_len: row.len() as i16,
                    pos: range.start,
                    end: range.end
                }
            }
        }
    }

    fn row_bytes_rev(&self, row: i16, range: Range<i16>) -> Self::BytesIterRev {
        if row < 0 {
            BytesIterRev {
                row: ::core::ptr::null(),
                row_len: 0,
                pos: range.end - 1,
                end: range.start
            }
        } else {
            match self.row(row) {
                Err(_) => BytesIterRev {
                    row: ::core::ptr::null(),
                    row_len: 0,
                    pos: range.end - 1,
                    end: range.start
                },
                Ok(row) => BytesIterRev {
                    row: row.as_ptr(),
                    row_len: row.len() as i16,
                    pos: range.end - 1,
                    end: range.start
                }
            }
        }
    }
}

pub struct BytesIter {
    row: *const u8,
    row_len: i16,
    pos: i16,
    end: i16,
}

impl Iterator for BytesIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.end {
            None
        } else {
            let result = if self.row.is_null() {
                0
            } else {
                if self.pos < 0 || self.pos >= self.row_len {
                    0
                } else {
                    unsafe {*self.row.offset(self.pos as isize)}
                }
            };

            self.pos += 1;

            Some(result)
        }
    }
}

pub struct BytesIterRev {
    row: *const u8,
    row_len: i16,
    pos: i16,
    end: i16,
}

impl Iterator for BytesIterRev {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.end {
            None
        } else {
            let result = if self.row.is_null() {
                0
            } else {
                if self.pos < 0 || self.pos >= self.row_len {
                    0
                } else {
                    let mut b = unsafe {*self.row.offset(self.pos as isize)};

                    b = (b & 0xF0) >> 4 | (b & 0x0F) << 4;
                    b = (b & 0xCC) >> 2 | (b & 0x33) << 2;
                    b = (b & 0xAA) >> 1 | (b & 0x55) << 1;

                    b
                }
            };

            self.pos -= 1;

            Some(result)
        }
    }
}

pub struct FlipH<'f, I> where I: DataBits {
    inner: &'f I,
}

impl<'f, I> DataBits for FlipH<'f, I> where I: DataBits {
    type BytesIter = I::BytesIterRev;
    type BytesIterRev = I::BytesIter;

    fn row_bits_len(&self) -> i16 {
        self.inner.row_bits_len()
    }

    fn row_bytes(&self, row: i16, range: Range<i16>) -> Self::BytesIter {
        self.inner.row_bytes_rev(row, -range.end..-range.start)
    }

    fn row_bytes_rev(&self, row: i16, range: Range<i16>) -> Self::BytesIterRev {
        self.inner.row_bytes(row, -range.end..-range.start)
    }
}

pub struct OffsetBytes<'f, I> where I: DataBits {
    inner: &'f I,
    offset_x: i16,
    offset_y: i16,
}

impl<'f, I> DataBits for OffsetBytes<'f, I> where I: DataBits {
    type BytesIter = I::BytesIter;
    type BytesIterRev = I::BytesIterRev;

    fn row_bits_len(&self) -> i16 {
        self.inner.row_bits_len()
    }

    fn row_bytes(&self, row: i16, range: Range<i16>) -> Self::BytesIter {
        self.inner.row_bytes(row - self.offset_y, range.start - self.offset_x..range.end - self.offset_x)
    }

    fn row_bytes_rev(&self, row: i16, range: Range<i16>) -> Self::BytesIterRev {
        self.inner.row_bytes_rev(row - self.offset_y, range.start - self.offset_x..range.end - self.offset_x)
    }
}

pub struct Rotate90<'f, I> where I: DataBits {
    inner: &'f I,
}

impl<'f, I> DataBits for Rotate90<'f, I> where I: DataBits {
    type BytesIter = RotateIter<I>;
    type BytesIterRev = I::BytesIterRev;

    fn row_bits_len(&self) -> i16 {
        self.inner.row_bits_len()
    }

    fn row_bytes(&self, row: i16, range: Range<i16>) -> RotateIter<I> {
        let (col, col_bit) = byte_and_bit_for_bit_index(row);

        RotateIter {
            inner: self.inner,
            row_start: range.start * 8,
            row_end: range.end * 8,
            col_byte: col,
            col_bit_mask: 0b1000_0000 >> col_bit,
            row_inc: 1
        }
    }

    fn row_bytes_rev(&self, _row: i16, _range: Range<i16>) -> Self::BytesIterRev {
        unimplemented!()
    }
}

pub struct RotateIter<I> where I: DataBits {
    inner: *const I,
    row_start: i16,
    row_end: i16,
    col_byte: i16,
    col_bit_mask: u8,
    row_inc: i8,
}

impl<I> Iterator for RotateIter<I> where I: DataBits {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_start == self.row_end {
            None
        } else {
            let mut current_row = self.row_start + (7 * self.row_inc) as i16;
            let mut output: u8 = 0;

            loop {
                let byte: u8 = unsafe {&*self.inner}.row_bytes(current_row, self.col_byte..self.col_byte+1).next().unwrap_or(0);
                output |= if (byte & self.col_bit_mask) > 0 { 1 } else { 0 };

                if current_row == self.row_start {
                    break;
                }

                current_row -= self.row_inc as i16;
                output <<= 1;
            }

            self.row_start += (8 * self.row_inc) as i16;

            Some(output)
        }
    }
}

fn byte_and_bit_for_bit_index(bit: i16) -> (i16, u8) {
    if bit < 0 {
        let full_bits = bit / 8;
        let remainder = -bit % 8;
        if remainder == 0 {
            return (full_bits, 0);
        } else {
            return (full_bits - 1, 8 - remainder as u8);
        }
    }

    (bit / 8, (bit % 8) as u8)
}

#[cfg(test)]
mod test {
    use super::*;
    use bitcanvas::BitCanvas;
    use bitcanvas::consts::{W8, H8};

    #[test]
    fn test_canvas_flip_h_and_offset_by_1() {
        let mut canvas: BitCanvas<W8, H8> = BitCanvas::<W8, H8>::new(8, 8).unwrap();
        canvas.row_mut(7).unwrap().copy_from_slice(&[0b1010_1100]);
        canvas.row_mut(3).unwrap().copy_from_slice(&[0b1110_1000]);

        let flip_h = canvas.flip_h();
        let flip_h_and_offset_bytes = flip_h.offset_bytes(1, 0);
        assert_eq!(flip_h_and_offset_bytes.row_bytes(7, 0..1).next(), Some(0b0011_0101));
        assert_eq!(flip_h_and_offset_bytes.row_bytes(3, 0..1).next(), Some(0b0001_0111));
    }

    #[test]
    fn test_canvas_offset_by_minus_1_and_flip_h() {
        let mut canvas: BitCanvas<W8, H8> = BitCanvas::<W8, H8>::new(8, 8).unwrap();
        canvas.row_mut(7).unwrap().copy_from_slice(&[0b1010_1100]);
        canvas.row_mut(3).unwrap().copy_from_slice(&[0b1110_1000]);

        let offset_bytes = canvas.offset_bytes(-1, 0);
        let offset_bytes_and_flip_h = offset_bytes.flip_h();
        assert_eq!(offset_bytes_and_flip_h.row_bytes(7, 0..1).next(), Some(0b0011_0101));
        assert_eq!(offset_bytes_and_flip_h.row_bytes(3, 0..1).next(), Some(0b0001_0111));
    }

    #[test]
    fn test_canvas_rotate_90() {
        let mut canvas: BitCanvas<W8, H8> = BitCanvas::<W8, H8>::new(8, 8).unwrap();
        canvas.row_mut(7).unwrap().copy_from_slice(&[0b0000_0001]);
        canvas.row_mut(6).unwrap().copy_from_slice(&[0b0000_0010]);
        canvas.row_mut(5).unwrap().copy_from_slice(&[0b0000_0100]);
        canvas.row_mut(4).unwrap().copy_from_slice(&[0b0000_1000]);
        canvas.row_mut(3).unwrap().copy_from_slice(&[0b0001_0000]);
        canvas.row_mut(2).unwrap().copy_from_slice(&[0b0010_0000]);
        canvas.row_mut(1).unwrap().copy_from_slice(&[0b0101_0011]);
        canvas.row_mut(0).unwrap().copy_from_slice(&[0b1111_1111]);

        let rotate90 = canvas.rotate_90();
        assert_eq!(rotate90.row_bytes(7, 0..1).next(), Some(0b1000_0011));
        assert_eq!(rotate90.row_bytes(6, 0..1).next(), Some(0b0100_0011));
        assert_eq!(rotate90.row_bytes(5, 0..1).next(), Some(0b0010_0001));
        assert_eq!(rotate90.row_bytes(4, 0..1).next(), Some(0b0001_0001));
        assert_eq!(rotate90.row_bytes(3, 0..1).next(), Some(0b0000_1011));
        assert_eq!(rotate90.row_bytes(2, 0..1).next(), Some(0b0000_0101));
        assert_eq!(rotate90.row_bytes(1, 0..1).next(), Some(0b0000_0011));
        assert_eq!(rotate90.row_bytes(0, 0..1).next(), Some(0b0000_0001));
    }

    #[test]
    fn check_positive_byte_and_bit() {
        assert_eq!((0, 0), byte_and_bit_for_bit_index(0));
        assert_eq!((0, 1), byte_and_bit_for_bit_index(1));
        assert_eq!((0, 7), byte_and_bit_for_bit_index(7));
        assert_eq!((1, 0), byte_and_bit_for_bit_index(8));
    }

    #[test]
    fn check_negative_byte_and_bit() {
        assert_eq!((-1, 7), byte_and_bit_for_bit_index(-1));
        assert_eq!((-1, 6), byte_and_bit_for_bit_index(-2));
        assert_eq!((-1, 5), byte_and_bit_for_bit_index(-3));
        assert_eq!((-1, 4), byte_and_bit_for_bit_index(-4));
        assert_eq!((-1, 3), byte_and_bit_for_bit_index(-5));
        assert_eq!((-1, 2), byte_and_bit_for_bit_index(-6));
        assert_eq!((-1, 1), byte_and_bit_for_bit_index(-7));
        assert_eq!((-1, 0), byte_and_bit_for_bit_index(-8));
        assert_eq!((-2, 7), byte_and_bit_for_bit_index(-9));
    }
}