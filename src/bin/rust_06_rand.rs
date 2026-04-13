use rand::random;

pub fn main() {
    let my_int_u8: u8 = random();
    let my_int_u64: u64 = random();
    println!("u8 is: {}, u64 is: {}", my_int_u8, my_int_u64);
}
