#![no_std]
#![no_main]

extern crate stm32g0;
//extern crate stm32g0xx_hal as hal;
//extern crate panic_semihosting;
extern crate panic_halt;
extern crate cortex_m;
extern crate cortex_m_rt;
//extern crate cortex_m_semihosting;

use cortex_m_rt::entry;
//use cortex_m_semihosting::hprintln;

//use hal::prelude::*;

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    let peripherals = stm32g0::stm32g0x1::Peripherals::take().unwrap();
    let mut rcc = peripherals.RCC;
    let mut port_a = peripherals.GPIOA;
    let mut tim2 = peripherals.TIM2;

    rcc.iopenr.modify(|_r, w| w.iopaen().bit(true));
    rcc.apbenr1.modify(|_r, w| w.tim2en().bit(true));

    configure_clock(&mut rcc);
    configure_mco(&mut port_a, &mut rcc);

    port_a.pupdr.modify(|_r, w| unsafe { w.pupdr1().bits(0) });
    port_a.moder.modify(|_r, w| unsafe {
        w
            .moder1().bits(MODE_OUTPUT)
    });
    port_a.ospeedr.modify(|_r, w| unsafe { w.ospeedr1().bits(OSPEED_VERY_HIGH) });
    port_a.otyper.modify(|_r, w| w.ot1().clear_bit());

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

fn configure_clock(rcc: &mut stm32g0::stm32g0x1::RCC) {
    rcc.pllsyscfgr.modify(|_r, w| unsafe { w.pllsrc().bits(0b10) });
    rcc.pllsyscfgr.modify(|_r, w| unsafe { w
        .pllm().bits(0b000)
        .plln().bits(0b0101)
        .pllr().bits(0b001)
    });
    rcc.pllsyscfgr.modify(|_r, w| unsafe { w
        .pllren().set_bit()
    });
    rcc.cr.modify(|_r, w| w.pllon().set_bit());
    let mut counter = 0;
    while !rcc.cr.read().pllrdy().bit_is_set() {
//        hprintln!("waiting for PLL");
        counter += 1;
        if counter > 10 {
//            hprintln!("PLL fail!");
            return;
        }
    }
//    hprintln!("PLL ready!");

    rcc.cfgr.modify(|_r, w| unsafe { w.sw().bits(0b010) });

    counter = 0;
    while rcc.cfgr.read().sws().bits() != 0b010 {
//        hprintln!("waiting for SYSCLK");
        counter += 1;
        if counter > 10 {
//            hprintln!("SYSCLK fail!");
            return;
        }
    }
//    hprintln!("SYSCLK ready!");
}

fn configure_mco(port_a: &mut stm32g0::stm32g0x1::GPIOA, rcc: &mut stm32g0::stm32g0x1::RCC) {
    port_a.afrh.modify(|_r, w| unsafe { w.afsel8().bits(0) });
    port_a.otyper.modify(|_r, w| w.ot8().clear_bit());
    port_a.pupdr.modify(|_r, w| unsafe { w.pupdr8().bits(0) });
    port_a.ospeedr.modify(|_r, w| unsafe { w.ospeedr8().bits(OSPEED_VERY_HIGH) });
    port_a.moder.modify(|_r, w| unsafe { w.moder8().bits(MODE_AF) });

    rcc.cfgr.modify(|r, w| unsafe {
        let mcopre_value: u8 = 0b000;

        let mut bits = r.bits();
        bits &= !((7 as u32) << 28);
        bits |= ((mcopre_value & 7) as u32) << 28;

        w.bits(bits);
        w.mcosel().bits(MCOSEL_SYSCLK)
    });
}

fn pause(tim2: &mut stm32g0::stm32g0x1::TIM2) {
    tim2.egr.write(|w| w.ug().set_bit());
    tim2.cr1.write(|w| w.cen().set_bit());
    while tim2.cnt.read().cnt_l().bits() < 0xFFFF {}
    tim2.cr1.write(|w| w.cen().clear_bit());
}

const MCOSEL_DIS: u8 = 0b000;
const MCOSEL_SYSCLK: u8 = 0b001;
const MCOSEL_RESERVED: u8 = 0b010;
const MCOSEL_HSI16: u8 = 0b011;
const MCOSEL_HSE: u8 = 0b100;
const MCOSEL_PLLRCLK: u8 = 0b101;
const MCOSEL_LSI: u8 = 0b110;
const MCOSEL_LSE: u8 = 0b111;

const OSPEED_VERY_LOW: u8 = 0b00;
const OSPEED_LOW: u8 = 0b01;
const OSPEED_HIGH: u8 = 0b00;
const OSPEED_VERY_HIGH: u8 = 0b11;

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