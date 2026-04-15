#[derive(Debug)]
enum StudentEnum {
    Name(String),
    Age(u8),
}

#[derive(Debug)]
struct MyStruct {
    name: String,
    age: u8,
}

pub fn main() {
    let my_str: String = "iamstring".to_string();
    my_print(&my_str);

    let my_int: u8 = 100;
    my_print(&my_int);

    let my_flt: f32 = 50.50;
    my_print(&my_flt);

    let my_vec: Vec<u8> = vec![1, 2];
    my_print(&my_vec);

    let std_enum = StudentEnum::Name("Mg Mg".to_string());
    my_print(&std_enum);

    let my_struct = MyStruct {
        name: "mg mg".to_string(),
        age: 29,
    };
    my_print(&my_struct);
}

fn my_print<T: std::fmt::Debug>(my_val: &T) {
    println!("{:?}", my_val);
}
