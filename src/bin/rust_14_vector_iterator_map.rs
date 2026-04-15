pub fn main() {
    let my_vec: Vec<u8> = vec![0, 1, 2];
    let mut my_map = my_vec.iter().map(|i| {
        let v = i + i;
        println!("{v}");
    });
    my_map.next();
    my_map.next();
    my_map.next();
}
