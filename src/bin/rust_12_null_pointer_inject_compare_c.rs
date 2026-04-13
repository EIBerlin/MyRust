use std::io::stdin;

pub fn main() {
    let mut my_input: String = Default::default(); // sdfg \0 dfghj
    stdin().read_line(&mut my_input).unwrap();
    println!("{my_input}");
}