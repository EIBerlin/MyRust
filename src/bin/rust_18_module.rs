mod rust_18_my_module;

use rust_18_my_module::{
    my_enum::MyEnum::{FIRST, SECOND},
    my_print::{main as my_print_main, print_me, print_you},
    my_struct::MyStruct,
};

pub fn main() {
    // my_print
    my_print_main();
    print_me();
    print_you();
    // my_struct
    let name = String::from("Zak");
    let age: u8 = 30;
    let my_struct = MyStruct { name, age };
    my_struct.print_name_and_age();
    // my_enum
    let my_first = FIRST;
    println!("my_first is first, {} ", my_first.is_first());
    println!("my_first is second, {} ", my_first.is_second());
    let my_second = SECOND;
    println!("my_second is first, {} ", my_second.is_first());
    println!("my_second is second, {} ", my_second.is_second());
}
