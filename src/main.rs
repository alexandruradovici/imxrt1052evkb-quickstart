//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

extern crate panic_halt;

use core::fmt::Write;
use core::ptr;

use cortex_m_rt::{entry, exception, ExceptionFrame};
use imxrt_ral::{read_reg, write_reg, modify_reg, reset_reg};
use cortex_m_semihosting::{debug, hprintln, hio};
use imxrt_ral::{gpio};

fn set_pin_9(gpio: &gpio::RegisterBlock) {
    write_reg!(gpio, gpio, GDIR, 1<<9);
    write_reg!(gpio, gpio, DR, 1<<9);
}

#[entry]
fn main() -> ! {
    // unsafe {
    //     ptr::read_volatile(0x3FFF_FFFE as *const u32);
    // }
    hprintln!("Hello, world!").unwrap();
    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    // debug::exit(debug::EXIT_SUCCESS);

    let gpio1 = gpio::GPIO1::take().unwrap();
    gpio1.write(|w| w.dr1().set_bit());
    // if let Some(instance) = gpio1 {
    //     hprintln!("gpio").unwrap();
    // }
    // else
    // {
    //     hprintln!("no gpio").unwrap();
    // }
    // let r = read_reg! (gpio, &gpio1, GDIR, GDIR);
    // gpio::GPIO1::release(gpio1);

    // set_pin_9 (&gpio1);
    loop {}
}
