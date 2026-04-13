#[derive(Debug)]


struct MyStruct {
    name: String,
}

pub fn main() {
    let my_struct = MyStruct{name: "Mg Mg".into()};
    println!("{}", my_struct.name.to_string());
    println!("{:#?}", my_struct);
}