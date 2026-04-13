use std::any::type_name;

fn type_of<T>(_: &mut T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let my_int: i8 = 12;
    let mut my_string = my_int.to_string();
    println!("{}", type_of(&mut my_string));
    my_fun(&mut my_string);
    println!("{my_string}");
    my_fun(&mut my_string);
}

fn my_fun(my_string: &mut String) {
    println!("{}", my_string);
    *my_string = "Iammuttable".to_string();
}