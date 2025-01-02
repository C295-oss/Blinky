#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");

    loop {
        rprintln!("Echo...");
        for _ in 0..100_000 {
            nop();
        }
    }
}
