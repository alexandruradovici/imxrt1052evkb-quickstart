//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use imxrt_ral::{read_reg, write_reg, modify_reg, reset_reg};
use cortex_m_semihosting::{debug, hprintln};
use imxrt_ral::{gpio};

fn set_pin_9(gpio: &gpio::RegisterBlock) {
    write_reg!(gpio, gpio, GDIR, 1<<9);
    write_reg!(gpio, gpio, DR, 1<<9);
}

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();
    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    // debug::exit(debug::EXIT_SUCCESS);

    let gpio1 = gpio::GPIO1::take().unwrap();
    set_pin_9 (&gpio1);
    loop {}
}
