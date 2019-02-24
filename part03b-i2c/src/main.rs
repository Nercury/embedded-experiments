#![deny(unsafe_code)]
#![no_main]
#![no_std]

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
use cortex_m::asm;
use stm32f1xx_hal as hal;
use nb::block;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use is31fl;

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

    let mut i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
//        Mode::Fast {
//            frequency: 400_000,
//            duty_cycle: DutyCycle::Ratio2to1,
//        },
        Mode::Standard {
            frequency: 100_000,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );
    let bus = shared_bus::CortexMBusManager::new(i2c);

    let mut matrix_device = is31fl::Device::<is31fl::Address11, _>::with_default_config(bus.acquire());
    matrix_device.set_current(is31fl::config::Current::Current20mA).unwrap();
    matrix_device.modify_config(|c|
        c
            .set_display_mode(is31fl::config::ConfigDisplayMode::Matrix1and2)
    ).unwrap();

    let mut matrix_device2 = is31fl::Device::<is31fl::Address01, _>::with_default_config(bus.acquire());
    matrix_device2.set_current(is31fl::config::Current::Current20mA).unwrap();
    matrix_device2.modify_config(|c|
        c
            .set_display_mode(is31fl::config::ConfigDisplayMode::Matrix1and2)
    ).unwrap();

    let mut led = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);

    let mut timer = Timer::syst(cp.SYST, 10.hz(), clocks);

    let mut index: u8 = 0;
    loop {

        matrix_device.set_matrix1_columns_rows(index, &[0b00000000]).unwrap();
        matrix_device.set_matrix2_columns_rows(index, &[0b00000000]).unwrap();
        matrix_device2.set_matrix1_columns_rows(index, &[0b00000000]).unwrap();
        matrix_device2.set_matrix2_columns_rows(index, &[0b00000000]).unwrap();
        index += 1;
        if index == 8 {
            index = 0;
        }
        matrix_device.set_matrix1_columns_rows(index, &[0b11111111]).unwrap();
        matrix_device.set_matrix2_columns_rows(index, &[0b11111111]).unwrap();
        matrix_device2.set_matrix1_columns_rows(index, &[0b11111111]).unwrap();
        matrix_device2.set_matrix2_columns_rows(index, &[0b11111111]).unwrap();
        matrix_device.update().unwrap();
        matrix_device2.update().unwrap();

        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
    }
}

#[exception]
fn HardFault(_ef: &cortex_m_rt::ExceptionFrame) -> ! {
    asm::bkpt();

    loop {
        asm::wfi();
    }
}

#[exception]
fn DefaultHandler(_irqn: i16) {
    loop {
        asm::wfi();
    }
}