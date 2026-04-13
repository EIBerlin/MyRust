pub fn main() {
    let my_int: i8 = 10;
    {
        let my_int: i8 = 20 + my_int;
        println!("{my_int}");
    }
    println!("{my_int}");
}