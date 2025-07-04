#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{get_time, yield_};

#[unsafe(no_mangle)]


fn main() ->i32 {
    let a = 1.2;
    let b = 3.4;
    println!("a = {}, b = {}", a, b);
    let sum = a + b;
    println!("------------------------sum = {}", sum);
    0
}