trait MyTrait {
    fn new();
}

struct MyStruct {
    //
}

impl MyTrait for MyStruct {
    fn new() {
        println!("I am output from my trait.");
    }
}

pub fn main() {
    MyStruct::new();
}
