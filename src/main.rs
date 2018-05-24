extern crate tinman;

fn main() {
    println!("Hello, world!");

    let test_name = "Register A";

    let my_register = tinman::register::Register::new(7, &test_name[..]);
    println!("The number is {}, and the test name is {}, which should match {}", my_register.num, my_register.register_name, my_register.chip.name);
}
