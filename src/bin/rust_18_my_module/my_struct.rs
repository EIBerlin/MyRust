pub struct MyStruct {
    pub name: String,
    pub age: u8,
}

impl MyStruct {
    pub fn print_name_and_age(self) {
        println!("name is {}, age is {}", self.name, self.age);
    }
}
