#![no_main]
#![no_std]

// use core::mem::take;

use panic_halt as _;

// use cortex_m::peripheral::{self, syst, Peripherals};
use cortex_m_rt::entry;
use stm32f4::stm32f407;
use rtt_target::rtt_init_print;

fn delay() {
    for _i in 0..10_000 {}
    // Does nothing...
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Gives access to the hardware's registers
    let peripherals = stm32f407::Peripherals::take().unwrap();

    // Manages clock for hardware peripherals
    let rcc = &peripherals.RCC;

    // Controls GPIO pins for Port D -- for LEDs.
    let gpiod = &peripherals.GPIOD;
    
    // Enable clock for GPIOD
    rcc.ahb1enr.modify(|_,w| {
        w.gpioden().set_bit();
        w.gpioaen().set_bit()
    });

    // Enable clock for SYSCFG
    rcc.apb2enr.modify(|_,w| w.syscfgen().set_bit());

    // Set pins PD12-PD15 as output
    gpiod.moder.modify(|_,w| {
        w.moder12().output();
        w.moder13().output();
        w.moder14().output();
        w.moder15().output()
    });

    loop {
        gpiod.odr.modify(|_,w| {
            w.odr12().set_bit();
            w.odr13().set_bit();
            w.odr14().set_bit();
            w.odr15().set_bit()

        });
        delay();

        gpiod.odr.modify(|_,w| {
            w.odr12().clear_bit();
            w.odr13().clear_bit();
            w.odr14().clear_bit();
            w.odr15().clear_bit()

        });
        delay();
    }
}
