#![deny(unsafe_code)]
#![no_main]
#![no_std]

//extern crate panic_halt;
extern crate panic_semihosting;

use stm32f1xx_hal::{
    prelude::*,
    pac,
    timer::Timer,
    i2c::{
        BlockingI2c,
        DutyCycle,
        Mode
    }
};
use nb::block;
use cortex_m_rt::{entry, exception};
use is31fl3730 as isd;
use isd::display::OutputRows;
use bitcanvas::BitCanvas;
use bitcanvas::consts::*;
use embedded_graphics::prelude::*;
use profont::ProFont7Point;
use is31fl3730::pixels::DataBits;

mod board {
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
        m2: isd::Device<I2C2>,
    }

    impl<I2C1, E1, I2C2, E2> Screen<I2C1, E1, I2C2, E2>
        where
            E1: core::fmt::Debug,
            E2: core::fmt::Debug,
            I2C1: hal::blocking::i2c::Write<Error = E1>,
            I2C2: hal::blocking::i2c::Write<Error = E2>,
    {
        pub fn new(i2c1: I2C1, i2c2: I2C2) -> Screen<I2C1, E1, I2C2, E2> {
            let mut screen = Screen {
                m1: isd::Device::new(isd::Address::Address11, i2c1),
                m2: isd::Device::new(isd::Address::Address01, i2c2),
            };

            screen.m1.modify_lighting(|c|
                c.set_current(isd::LightingCurrent::Current10mA)).expect("init1");
            screen.m1.modify_config(|c|
                c.set_display_mode(isd::ConfigDisplayMode::Matrix1and2)).unwrap();

            screen.m2.modify_lighting(|c|
                c.set_current(isd::LightingCurrent::Current10mA)).expect("init2");
            screen.m2.modify_config(|c|
                c.set_display_mode(isd::ConfigDisplayMode::Matrix1and2)).unwrap();

            screen
        }

        pub fn render<C>(&mut self, canvas: &C) where C: DataBits {
            isd::display::MatrixTargetPrimary8x8{}
                .output_pixels(&mut self.m1,
                               &canvas
                                   .flip_h()
                                   .offset_bytes(1, 0)
                ).expect("set-1-1");
            isd::display::MatrixTargetSecondary8x8{}
                .output_pixels(&mut self.m1,
                               &canvas
                                   .offset_bytes(-1, 0)
                                   .rotate_90()
                ).expect("set-1-2");
            isd::display::MatrixTargetPrimary8x8{}
                .output_pixels(&mut self.m2,
                               &canvas
                                   .flip_h()
                                   .offset_bytes(3, 0)
                ).expect("set-2-1");
            isd::display::MatrixTargetSecondary8x8{}
                .output_pixels(&mut self.m2,
                               &canvas
                                   .offset_bytes(-3, 0)
                                   .rotate_90()
                ).expect("set-2-2");

            self.m1.update().expect("fail-1-u");
            self.m2.update().expect("fail-2-u");
        }
    }
}

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000,
            duty_cycle: DutyCycle::Ratio2to1,
        },
//        Mode::Standard {
//            frequency: 100_000,
//        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );
    let bus = shared_bus::CortexMBusManager::new(i2c);

    let mut screen = board::Screen::new(bus.acquire(), bus.acquire());
    let mut led = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    let mut timer = Timer::syst(cp.SYST, 5.hz(), clocks);

    let mut canvas = BitCanvas::<W32, H8>::new(32, 8).unwrap();
    canvas.draw(
        ProFont7Point::render_str(" Wut?!")
            .into_iter()
    );

    loop {
        screen.render(&canvas);

        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
    }
}