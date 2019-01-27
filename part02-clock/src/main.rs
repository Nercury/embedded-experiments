#![no_std]
#![no_main]

extern crate stm32f1;
//extern crate panic_semihosting;
extern crate panic_halt;
extern crate cortex_m_rt;
//extern crate cortex_m_semihosting;

use cortex_m_rt::{entry, exception};
//use cortex_m_semihosting::hprintln;

// used when invoking C code to configure system clock
// extern "C" {
//     fn HAL_Init();
//     fn HAL_IncTick();
//     fn SystemClock_Config();
// }

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {

    // used when invoking C code to configure system clock
    // unsafe { HAL_Init(); }
    // unsafe { SystemClock_Config(); }

    let peripherals = stm32f1::stm32f100::Peripherals::take().unwrap();
    let mut rcc = peripherals.RCC;
    let mut port_a = peripherals.GPIOA;

    configure_mco(&mut rcc, &mut port_a);
    configure_system_clock(&mut rcc);

    loop {

    }
}

fn configure_mco(rcc: &mut stm32f1::stm32f100::RCC, port: &mut stm32f1::stm32f100::GPIOA) {
    // enable port clock
    rcc.apb2enr.modify(|_r, w| w.iopaen().set_bit());
    // configure port mode and enable alternate function
    port.crh.modify(|_r, w| unsafe {
        w
            .mode8().bits(MODE_OUTPUT_50MHz)
            .cnf8().bits(CNF_AF_OUTPUT_PUSHPULL)
    });
    // enable MCO alternate function (PA8)
    rcc.cfgr.modify(|_r, w| w.mco().sysclk());
}

fn configure_system_clock(rcc: &mut stm32f1::stm32f100::RCC) {
    rcc.cr.modify(|_r, w| w.hseon().set_bit());
    while rcc.cr.read().hserdy().bit_is_clear() {}

    const RCC_CFGR2_DIV2: u8 = 0b0001; // bits = divider - 1 (1 = 2 - 1)
    rcc.cfgr2.modify(|_r, w| unsafe { w.prediv1().bits(RCC_CFGR2_DIV2)});
    rcc.cfgr.modify(|_r, w| w.pllsrc().hse_div_prediv());

    rcc.cfgr.modify(|_r, w| w.pllmul().mul6());
    rcc.cr.modify(|_r, w| w.pllon().set_bit());
    while rcc.cr.read().pllrdy().bit_is_clear() {}

    rcc.cfgr.modify(|_r, w| w.sw().pll());
    while !rcc.cfgr.read().sws().is_pll() {}
}

// used when invoking C code to configure system clock
//
// #[exception]
// fn SysTick() {
//     unsafe { HAL_IncTick() }
// }

const MODE_INPUT: u8 = 0b00;
const MODE_OUTPUT_10MHz: u8 = 0b01;
const MODE_OUTPUT_2MHz: u8 = 0b10;
const MODE_OUTPUT_50MHz: u8 = 0b11;

const CNF_OUTPUT_PUSHPULL: u8 = 0b00;
const CNF_INPUT_FLOATING: u8 = 0b01;
const CNF_AF_OUTPUT_PUSHPULL: u8 = 0x00000002;