pub fn main() {
    let my_vec: Vec<u8> = vec![0, 1, 2];
    let mut my_compare: u8 = 0;
    for i in my_vec {
        println!("i is : {i}");
        assert_eq!(i, my_compare);
        my_compare += 1;
    }
}