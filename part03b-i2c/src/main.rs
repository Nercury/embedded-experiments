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
        //DutyCycle,
        Mode
    }
};
use nb::block;
use cortex_m_rt::{entry, exception};
use is31fl3730 as isd;

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

    let mut matrix_device = isd::Device::new(isd::Address::Address11, bus.acquire());
    matrix_device.modify_lighting(|c|
        c.set_current(isd::LightingCurrent::Current5mA)).expect("init1");
    matrix_device.modify_config(|c|
        c.set_display_mode(isd::ConfigDisplayMode::Matrix1and2)).unwrap();

    let mut matrix_device2 = isd::Device::new(isd::Address::Address01, bus.acquire());
    matrix_device2.modify_lighting(|c|
        c.set_current(isd::LightingCurrent::Current5mA)).expect("init2");
    matrix_device2.modify_config(|c|
        c.set_display_mode(isd::ConfigDisplayMode::Matrix1and2)).unwrap();

    let mut led = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);

    let mut timer = Timer::syst(cp.SYST, 10.hz(), clocks);

    let mut index: u8 = 0;
    loop {

        matrix_device.set_matrix1_columns_rows(index, &[0b00000000]).expect("fail-1-1");
        matrix_device.set_matrix2_columns_rows(index, &[0b00000000]).expect("fail-1-2");
        matrix_device2.set_matrix1_columns_rows(index, &[0b00000000]).expect("fail-2-1");
        matrix_device2.set_matrix2_columns_rows(index, &[0b00000000]).expect("fail-2-2");
        index += 1;
        if index == 8 {
            index = 0;
        }
        matrix_device.set_matrix1_columns_rows(index, &[0b11111111]).expect("fail-1-1");
        matrix_device.set_matrix2_columns_rows(index, &[0b11111111]).expect("fail-1-2");
        matrix_device2.set_matrix1_columns_rows(index, &[0b11111111]).expect("fail-2-1");
        matrix_device2.set_matrix2_columns_rows(index, &[0b11111111]).expect("fail-2-2");
        matrix_device.update().expect("fail-1-u");
        matrix_device2.update().expect("fail-2-u");

        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
    }
}