#![no_main]
#![no_std]

#[macro_use]
extern crate pg;

use pg::peripheral;

#[export_name = "main"]
#[inline(never)]
pub extern "C" fn main() -> ! {
    let usart1 = unsafe { peripheral::usart1_mut() };

    // Send a single character
    usart1.tdr.write(|w| w.tdr(u16::from('X' as u8)));

    loop {}
}
