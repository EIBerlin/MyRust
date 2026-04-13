use std::{io::stdin, any::type_name};

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn main() {
    let mut my_input: String = Default::default();
    let _ = stdin().read_line(&mut my_input).unwrap();
    let type_check: f32 = my_input.trim().parse().expect("Please enter a valid number");
    println!("my_input val is :: {}", type_check);
    println!("my_input typ is :: {}", type_of(type_check));
    assert_eq!(1., type_check);
}