#![no_std]
#![no_main]

extern crate pg;

#[export_name = "main"]
#[inline(never)]
pub extern "C" fn main() -> ! {
    unsafe {
        // A magic address!
        const GPIOE_BSRR: u32 = 0x48001018;

        // Turn on the "North" LED (red)
        *(GPIOE_BSRR as *mut u32) = 1 << 9;

        // Turn on the "East" LED (green)
        *(GPIOE_BSRR as *mut u32) = 1 << 11;

        // Turn off the "North" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);

        // Turn on the "East" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
    }

    loop {}
}
