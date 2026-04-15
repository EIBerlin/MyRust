trait MyTrait /*: std::fmt::Debug -> with SuperTrait*/ {
    fn new(&self)
    where
        Self: std::fmt::Debug, /*with where clause trait -> fn trait*/
    {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
struct MyStruct;

#[derive(Debug)]
enum MyEnum {
    Me, /*at least one*/
}

impl MyTrait for MyStruct {}

impl MyTrait for MyEnum {}

pub fn main() {
    let my_struct = MyStruct;
    my_struct.new();

    let my_enum = MyEnum::Me;
    my_enum.new();
}
