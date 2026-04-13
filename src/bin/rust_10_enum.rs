#[derive(PartialEq)]

enum MyEnum {
    Success,
}

pub fn main() {
    let my_success = MyEnum::Success;
    if my_success == MyEnum::Success {
        println!("success!");
    }
}