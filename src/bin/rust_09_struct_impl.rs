#[derive(Debug)]

struct MyStruct {
    a: u8,
    b: u8,
}

impl MyStruct {
    fn plus(&self) { // Not Ownership Move
        let c = self.a + self.b;
        println!("{c}");
    }
}

pub fn main() {
    let my_struct = MyStruct{a: 1, b: 2};
    my_struct.plus(); // Automatic Referencing & Dereferencing
    // MyStruct::plus(&my_struct);
}