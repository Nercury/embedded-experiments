#![no_std]
#![no_main]

extern crate stm32g0;
//extern crate panic_semihosting;
extern crate panic_halt;
extern crate cortex_m;
extern crate cortex_m_rt;
//extern crate cortex_m_semihosting;

use cortex_m_rt::entry;
//use cortex_m_semihosting::hprintln;

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    let peripherals = stm32g0::stm32g0x1::Peripherals::take().unwrap();
    let rcc = peripherals.RCC;
    let port_a = peripherals.GPIOA;
    let mut tim2 = peripherals.TIM2;

    rcc.iopenr.write(|w| w.iopaen().bit(true));
    rcc.apbenr1.write(|w| w.tim2en().bit(true));

    port_a.moder.write(|w| unsafe {
        w
            .moder1().bits(MODE_OUTPUT)
    });

    loop {
        port_a.bsrr.write(|w|
            w
                .bs1().set_bit()
        );
        pause(&mut tim2);
        port_a.brr.write(|w|
            w
                .br1().set_bit()
        );
        pause(&mut tim2);
    }
}

fn pause(tim2: &mut stm32g0::stm32g0x1::TIM2) {
    tim2.egr.write(|w| w.ug().set_bit());
    tim2.cr1.write(|w| w.cen().set_bit());
    while tim2.cnt.read().cnt_l().bits() < 0xFFFFFF {}
    tim2.cr1.write(|w| w.cen().clear_bit());
}

const MODE_INPUT: u8 = 0b00;
const MODE_OUTPUT: u8 = 0b01;
const MODE_AF: u8 = 0b10;
const MODE_ANALOG: u8 = 0b11;

const CNF_OUTPUT_PUSHPULL: u8 = 0b00;
const CNF_INPUT_FLOATING: u8 = 0b01;
const CNF_AF_OUTPUT_PUSHPULL: u8 = 0b10;

const RCC_CFGR2_DIV1: u8 = 0b0000;
const RCC_CFGR2_DIV2: u8 = 0b0001;
const RCC_CFGR2_DIV3: u8 = 0b0010;
const RCC_CFGR2_DIV4: u8 = 0b0011;
const RCC_CFGR2_DIV5: u8 = 0b0100;
const RCC_CFGR2_DIV6: u8 = 0b0101;
const RCC_CFGR2_DIV7: u8 = 0b0110;
const RCC_CFGR2_DIV8: u8 = 0b0111;
const RCC_CFGR2_DIV9: u8 = 0b1000;
const RCC_CFGR2_DIV10: u8 = 0b1001;
const RCC_CFGR2_DIV11: u8 = 0b1010;
const RCC_CFGR2_DIV12: u8 = 0b1011;
const RCC_CFGR2_DIV13: u8 = 0b1100;
const RCC_CFGR2_DIV14: u8 = 0b1101;
const RCC_CFGR2_DIV15: u8 = 0b1110;
const RCC_CFGR2_DIV16: u8 = 0b1111;