#![no_std]
#![no_main]

extern crate stm32f1;
extern crate panic_semihosting;
//extern crate panic_halt;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    let peripherals = stm32f1::stm32f100::Peripherals::take().unwrap();
    let rcc = peripherals.RCC;
    let port_c = peripherals.GPIOC;

    rcc.apb2enr.write(|w| w.iopcen().bit(true));

    port_c.crh.write(|w| unsafe {
        w
            .mode8().bits(MODE_OUTPUT_10MHz)
            .cnf8().bits(CNF_OUTPUT_PUSHPULL)
            .mode9().bits(MODE_OUTPUT_10MHz)
            .cnf9().bits(CNF_OUTPUT_PUSHPULL)
    });

    port_c.bsrr.write(|w|
        w
            .bs8().set_bit()
            .bs9().set_bit()
    );

    loop {

    }
}

const MODE_INPUT: u8 = 0b00;
const MODE_OUTPUT_10MHz: u8 = 0b01;
const MODE_OUTPUT_2MHz: u8 = 0b10;
const MODE_OUTPUT_50MHz: u8 = 0b11;

const CNF_OUTPUT_PUSHPULL: u8 = 0b00;
const CNF_INPUT_FLOATING: u8 = 0b01;