#![no_std]
#![no_main]

use user_lib::print_task_info;

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]

fn main() -> i32 {
    println!("hello ");
    print_task_info();
    0
}