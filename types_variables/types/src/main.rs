#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn main() {
    let a: u8 = 123; // u = unsigned, 8 bits, 0 - 255
    println!("a = {}", a); // immutable
    // a = 456; error because its not declared as mutable

    let mut b: i8 = 0; // i = signed, 8 bits, -128 - 127
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    let mut c = 123456789; // i32 = 32 bits, 4 bytes
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);

    // u8, u16, u32, u65, i8, i16, ...

    // usize isize
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bits  OS", z, size_of_z, size_of_z*8);

    let d: char = 'x'; // 4 bytes, 32 bits
    println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));

    // f32 f64 IEEE754 signed

    let e: f32 = 2.5;
    println!("{}, size = {} bytes", e, mem::size_of_val(&e));

    let g: bool = false; // true
    println!("{}, size = {} bytes", g, mem::size_of_val(&g));
}
