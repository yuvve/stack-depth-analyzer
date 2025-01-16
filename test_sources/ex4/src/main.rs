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

// recursive
fn sum_recursive(n: u8) -> u32 {
    match n {
        0 => 0,
        _ => n as u32 + sum_recursive(n - 1),
    }
}

// iterative
fn sum_iterative(n: u8) -> u32 {
    let mut sum = 0;
    for v in 0..=n {
        sum += v as u32
    }
    sum
}

// mathematical formula
fn sum_formula(n: u8) -> u32 {
    let n: u32 = n as u32;
    n * (n + 1) / 2
}

#[no_mangle]
#[inline(never)]
// test sum_recursive == sum_formula
pub fn equal_formula_rec(n: u8) {
    assume(n < 10);
    assert!(sum_recursive(n) == sum_formula(n));
}

#[no_mangle]
#[inline(never)]
// test sum_iterative == sum_recursive
pub fn equal_iter_rec(n: u8) {
    assume(n < 10);
    assert!(sum_iterative(n) == sum_recursive(n));
}

// test complexity sum_recursive
#[no_mangle]
#[inline(never)]
pub fn complexity_sum_recursive(n: u8) {
    assume(n < 10);
    let _ = sum_recursive(n);
}

// test complexity sum_iterative
#[no_mangle]
#[inline(never)]
pub fn complexity_sum_iterative(n: u8) {
    assume(n < 10);
    let _ = sum_iterative(n);
}

// test complexity sum_formula
#[no_mangle]
#[inline(never)]
pub fn complexity_sum_formula(n: u8) {
    assume(n < 10);
    let _ = sum_formula(n);
}

#[entry]
fn main() -> ! {
    // force the result to be read, thus prevent LLVM to optimize out the `get_sign` function.
    unsafe {
        core::ptr::read_volatile(&complexity_sum_iterative(0));
        core::ptr::read_volatile(&complexity_sum_recursive(0));
        core::ptr::read_volatile(&complexity_sum_formula(0));
        core::ptr::read_volatile(&equal_iter_rec(0));
        core::ptr::read_volatile(&equal_formula_rec(0));
    }
    #[allow(clippy::empty_loop)]
    loop {}
}
