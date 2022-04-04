extern crate lib_demo;

// mod vars;
// mod stack_heap;
// mod ownership;
// mod generics;
// mod lifetime;
// mod structs;
// mod enums;
mod debug;
mod error_handling;
mod unit_test;

fn main() {
    // println!("Hello, world!");
    debug::run();
    // vars::run();
    // stack_heap::run();
    // ownership::run();
    // generics::run();
    // lifetime::run();
    // structs::run();
    // enums::run();
    error_handling::run();
    // vars::sub_a::func_a();
    // vars::sub_b::func_b();
    lib_demo::print_random_number();
}
