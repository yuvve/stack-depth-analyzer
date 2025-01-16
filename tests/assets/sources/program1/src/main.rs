//! cargo symex --elf --example ex4 --function <function to test>
//!
//! functional equivalence

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52840_hal as _;
use panic_halt as _;

#[allow(dead_code)]
use symex_lib::assume;

#[no_mangle]
#[inline(never)]
pub fn function1() -> [i32; 10]{
    let mut v: [i32; 10] = [0; 10];
    for i in 0..10 {
        v[i] = i as i32;
    }
    v
}

#[no_mangle]
#[inline(never)]
pub fn function2() -> [i32; 100]{
    let mut v: [i32; 100] = [0; 100];
    for i in 0..100 {
        v[i] = (i as i32)*2;
    }
    v
}

#[no_mangle]
#[inline(never)]
pub fn function3() -> [i32; 500]{
    let mut v: [i32; 500] = [0; 500];
    for i in 0..500 {
        v[i] = i as i32;
    }
    v
}

#[no_mangle]
#[inline(never)]
pub fn function4() -> [i32; 10]{
    let mut v: [i32; 10] = [0; 10];
    for i in 0..10 {
        v[i] = (i as i32)*4;
    }
    v
}

#[entry]
fn main() -> ! {
    // force the result to be read, thus prevent LLVM to optimize out the `get_sign` function.
    unsafe {
        core::ptr::read_volatile(&function1());
        core::ptr::read_volatile(&function2());
        core::ptr::read_volatile(&function3());
        core::ptr::read_volatile(&function4());
    }
    #[allow(clippy::empty_loop)]
    loop {}
}
