pub fn main() {
    let my_vec: Vec<u8> = vec![0, 1, 2];
    // let mut my_filter = my_vec.iter().filter(|i| i.to_owned().to_owned() == 0);
    let mut my_filter = my_vec.iter().filter(|&&i| i == 0);
    println!("{:?}", my_filter.next());
    println!("{:?}", my_filter.next());
    println!("{:?}", my_filter.next());
    println!("{:?}", my_filter.next());
}
