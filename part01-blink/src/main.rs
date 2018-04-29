#![no_std]

extern crate stm32f1;
extern crate panic_abort;

#[allow(non_upper_case_globals, dead_code)]
const MODE_INPUT: u8 = 0b00;
#[allow(non_upper_case_globals, dead_code)]
const MODE_OUTPUT_10MHz: u8 = 0b01;
#[allow(non_upper_case_globals, dead_code)]
const MODE_OUTPUT_2MHz: u8 = 0b10;
#[allow(non_upper_case_globals, dead_code)]
const MODE_OUTPUT_50MHz: u8 = 0b11;

#[allow(dead_code)]
const CNF_OUTPUT_PUSHPULL: u8 = 0b00;
#[allow(dead_code)]
const CNF_INPUT_FLOATING: u8 = 0b01;

fn main() {
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

    loop {}
}