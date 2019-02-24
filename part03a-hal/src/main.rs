#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

use stm32f1xx_hal::{
    prelude::*,
    pac,
    timer::Timer,
};
use nb::block;
use cortex_m_rt::{entry, exception, ExceptionFrame};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut led = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);

    let mut timer = Timer::syst(cp.SYST, 1.hz(), clocks);

    loop {
        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
    }
}