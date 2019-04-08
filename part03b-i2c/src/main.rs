#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate num_format;
extern crate is31fl3730;
//extern crate panic_halt;
extern crate panic_semihosting;

use num_format::{Buffer, Error, CustomFormat, Grouping};
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
use bitcanvas::BitCanvas;
use bitcanvas::consts::*;
use embedded_graphics::prelude::*;
use profont::ProFont7Point;

mod board;

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
    let mut timer = Timer::syst(cp.SYST, 10.hz(), clocks);

    let mut canvas = BitCanvas::<W32, H8>::new(32, 8).unwrap();

    let mut buffer = Buffer::new();
    let format = CustomFormat::builder()
        .grouping(Grouping::Standard)
        .minus_sign("-")
        .separator("")
        .build().unwrap();

    let mut value: i16 = 0;

    loop {
        buffer.write_formatted(&value, &format);
        canvas.draw(
            ProFont7Point::render_str( buffer.as_str())
                .into_iter()
        );
        value += 11;

        screen.render(&canvas);

        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
    }
}