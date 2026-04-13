use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn main() {
    let my_int: i8 = 100;
    println!(
        "before val is :: {:#?} , type is :: {:#?}",
        my_int,
        type_of(my_int)
    );
    let my_float = my_int as f32;
    println!("val is :: {:#?} , type is :: {:#?}", my_float, type_of(my_float));
}
