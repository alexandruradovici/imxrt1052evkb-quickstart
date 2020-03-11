//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

extern crate panic_halt;

// use core::fmt::Write;
// use core::ptr;

// use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m_rt::{entry};
use imxrt_ral::{read_reg, write_reg};
// modify_reg, reset_reg
// use cortex_m_semihosting::{debug, hprintln, hio};
use cortex_m_semihosting::{hprintln};
use imxrt_ral::{gpio, iomuxc, ccm};

// fn set_pin_9(gpio: &gpio::RegisterBlock) {
    // write_reg!(gpio, gpio, GDIR, 1<<9);
//     write_reg!(gpio, gpio, DR, 1<<9);
// }

#[entry]
fn main() -> ! {
    // unsafe {
    //     ptr::read_volatile(0x3FFF_FFFE as *const u32);
    // }
    hprintln!("Hello, world!").unwrap();
    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    // debug::exit(debug::EXIT_SUCCESS);

    let gpio = gpio::GPIO1::take().unwrap();
    let iomuxc = iomuxc::IOMUXC::take().unwrap();
    let ccm = ccm::CCM::take().unwrap();

    // write clock on iomuxc
    let mut val = read_reg!(ccm, ccm, CCGR4);
    val |= 5;
    write_reg!(ccm, ccm, CCGR4, val);

    val = read_reg!(iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_AD_B0_09) as u32;
    // val |= (((5 as u32) << 0 as u32) & 7);
    val |= 5;
    write_reg!(iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_AD_B0_09, val);

    val = read_reg!(iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_AD_B0_09);
    val |= 4272;
    write_reg!(iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_AD_B0_09, val);
    // let mut val = read_reg!(gpio, gpio, DR);
    // val &= 1<<9; 

    // write clock on gpio1
    val = read_reg!(ccm, ccm, CCGR1);
    val |= (1<<27) + (1 << 26);
    write_reg!(ccm, ccm, CCGR1, val);

    val = read_reg!(gpio, gpio, IMR);
    val &= !((1<<9) as u32);
    hprintln!("{:#032b}", val).unwrap();
    write_reg!(gpio, gpio, IMR, val);

    val = read_reg!(gpio, gpio, DR);
    hprintln!("{:#x?}", val).unwrap();
    val &= !((1<<9) as u32);
    hprintln!("{:#032b}", val).unwrap();
    write_reg!(gpio, gpio, DR, val);

    val = read_reg!(gpio, gpio, GDIR);
    hprintln!("{:#032b}", val).unwrap();
    val |= 1<<9;
    hprintln!("{:#032b}", val).unwrap();
    write_reg!(gpio, gpio, GDIR, val);
    // 0x401F82D0U
    // 0b1000010110000

    // SW_MUX_CTL_PAD_GPIO_AD_B0_09
    // iomuxc.write(|w| w.SW_MUX_CTL_PAD_GPIO_AD_B0_09().bits(3));
    // gpio1.write(|w| w.dr1().set_bit());
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
