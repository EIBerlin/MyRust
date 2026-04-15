trait MyTrait /*: std::fmt::Debug -> with SuperTrait*/ {
    fn new(&self)
    where
        Self: std::fmt::Debug, /*with where clause trait -> fn trait*/
    {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
struct MyStruct {
    name: String,
}

impl MyTrait for MyStruct {}

pub fn main() {
    let my_struct = MyStruct {
        name: "Mg Mg".to_string(),
    };
    my_struct.new();
}
