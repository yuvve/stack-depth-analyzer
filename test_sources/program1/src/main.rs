#![no_std]
#![no_main]
use cortex_m_rt::entry;
use nrf52840_hal as _;
use panic_halt as _;

#[entry]
fn main() -> !{
    let v1 = function1();
    let v2 = function2();
    loop {
        let var = v1[0] + v2[0];
    }
}

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