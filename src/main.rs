#![no_std]
#![no_main]

use cortex_m::asm;
//use cortex_m::asm;
//use cortex_m_semihosting::{debug, hprintln};
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

//use cortex_m::asm;
use cortex_m_rt::entry;
use volatile_register::RW;

// For Method 2: 'Create' your own volatile register
#[repr(C)]
pub struct PORT1 {
    pub io_pin: [RW<u16>; 1],
    pub dd_pin: [RW<u16>; 1],
}

#[entry]
fn main() -> ! {
    //asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    let perph = unsafe { ra4m1::Peripherals::steal() };

    //let port1 = 0x40040020 as *mut PORT1;
    unsafe {
        // For Method 2: 'Create' your own volatile register (Flip flop not implemented)
        //let mut reg = (*port1).dd_pin[0].read();
        //reg |= 1 << 12;
        //(*port1).dd_pin[0].write(reg);

        perph.PORT1.pdr().modify(|r, w| w.bits(r.bits() | 1 << 12));
        perph.PORT1.pdr().modify(|r, w| w.bits(r.bits() | 1 << 11));
    }

    let mut flip = false;
    loop {
        (1..1000000).for_each(|_| asm::nop());

        unsafe {
            // For Method 2: 'Create' your own volatile register (Flip flop not implemented)
            //let mut reg = (*port1).io_pin[0].read();
            //reg |= 1 << 12;
            //(*port1).io_pin[0].write(reg);

            if flip {
                perph.PORT1.podr().modify(|r, w| w.bits(r.bits() | 1 << 11));

                perph
                    .PORT1
                    .podr()
                    .modify(|r, w| w.bits(r.bits() & !(1 << 12)));

                flip = false;
            } else {
                perph.PORT1.podr().modify(|r, w| w.bits(r.bits() | 1 << 12));

                perph
                    .PORT1
                    .podr()
                    .modify(|r, w| w.bits(r.bits() & !(1 << 11)));

                flip = true;
            }
        }
    }
}
