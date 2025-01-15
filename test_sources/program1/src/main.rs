fn main() {
    println!("Hello, world!");
}

#[no_mangle]
#[inline(never)]
pub fn function1() -> Vec<i32>{
    let mut v = Vec::with_capacity(1000);
    for i in 0..1000 {
        v.push(i);
    }
    v
}

#[no_mangle]
#[inline(never)]
pub fn function2() -> Vec<i32>{
    let mut v = Vec::with_capacity(1000);
    for i in 0..1000 {
        v.push(i*3);
    }
    v
}