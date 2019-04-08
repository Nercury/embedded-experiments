use embedded_hal as hal;
use is31fl3730 as isd;
use isd::pixels::DataBits;
use isd::display::OutputRows;

pub struct Screen<I2C1, E1, I2C2, E2>
    where
        E1: core::fmt::Debug,
        E2: core::fmt::Debug,
        I2C1: hal::blocking::i2c::Write<Error = E1>,
        I2C2: hal::blocking::i2c::Write<Error = E2>,
{
    m1: isd::Device<I2C1>,
    m1_should_reload: bool,
    m2: isd::Device<I2C2>,
    m2_should_reload: bool,
}

fn restart<I2C, E>(device: &mut isd::Device<I2C>) -> Result<(), E>
    where
        E: core::fmt::Debug,
        I2C: hal::blocking::i2c::Write<Error = E>
{
    device.reset()?;
    device.modify_lighting(|c|
        c.set_current(isd::LightingCurrent::Current20mA))?;
    device.modify_config(|c|
        c.set_display_mode(isd::ConfigDisplayMode::Matrix1and2))?;
    Ok(())
}

impl<I2C1, E1, I2C2, E2> Screen<I2C1, E1, I2C2, E2>
    where
        E1: core::fmt::Debug,
        E2: core::fmt::Debug,
        I2C1: hal::blocking::i2c::Write<Error = E1>,
        I2C2: hal::blocking::i2c::Write<Error = E2>,
{
    pub fn new(i2c1: I2C1, i2c2: I2C2) -> Screen<I2C1, E1, I2C2, E2> {
        Screen {
            m1: isd::Device::new(isd::Address::Address11, i2c1),
            m1_should_reload: true,
            m2: isd::Device::new(isd::Address::Address01, i2c2),
            m2_should_reload: true,
        }
    }

    fn render1<C>(&mut self, canvas: &C) -> Result<(), E1> where C: DataBits {
        isd::display::MatrixTargetPrimary8x8{}
            .output_pixels(&mut self.m1,
                           &canvas
                               .flip_h()
                               .offset_bytes(1, 0)
            )?;
        isd::display::MatrixTargetSecondary8x8{}
            .output_pixels(&mut self.m1,
                           &canvas
                               .offset_bytes(-1, 0)
                               .rotate_90()
            )?;
        self.m1.update()?;
        Ok(())
    }

    fn render2<C>(&mut self, canvas: &C) -> Result<(), E2> where C: DataBits {
        isd::display::MatrixTargetPrimary8x8{}
            .output_pixels(&mut self.m2,
                           &canvas
                               .flip_h()
                               .offset_bytes(3, 0)
            )?;
        isd::display::MatrixTargetSecondary8x8{}
            .output_pixels(&mut self.m2,
                           &canvas
                               .offset_bytes(-3, 0)
                               .rotate_90()
            )?;
        self.m2.update()?;
        Ok(())
    }

    pub fn render<C>(&mut self, canvas: &C) where C: DataBits {
        if self.m1_should_reload {
            if let Ok(()) = restart(&mut self.m1) {
                self.m1_should_reload = self.render1(canvas).is_err();
            }
        } else {
            self.m1_should_reload = self.render1(canvas).is_err()
        }

        if self.m2_should_reload {
            if let Ok(()) = restart(&mut self.m2) {
                self.m2_should_reload = self.render2(canvas).is_err();
            }
        } else {
            self.m2_should_reload = self.render2(canvas).is_err()
        }
    }
}
