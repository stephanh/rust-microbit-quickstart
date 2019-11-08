#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;

use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    loop {}
}
