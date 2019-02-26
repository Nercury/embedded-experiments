#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
//extern crate panic_semihosting;
extern crate stm32g0xx_hal as hal;
extern crate nb;

use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;
use nb::block;
use rt::{entry, exception, ExceptionFrame};

#[entry]
fn main() -> ! {
    hal::debug::init();
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = hal::rcc::RccExt::constrain(dp.RCC);

    let gpioa = dp.GPIOA.split(&mut rcc);
    let mut led = gpioa.pa1.into_push_pull_output();

    //let mut timer = dp.TIM2.timer(&mut rcc);
    //timer.start(100.ms());

    led.toggle();

    loop {

//        block!(timer.wait()).unwrap();
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("Hard fault {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}