trait MyTrait {
    fn new() {
        println!("I am new of my trait.");
    }
}

struct MyStruct {}

impl MyTrait for MyStruct {}

pub fn main() {
    MyStruct::new();
}
