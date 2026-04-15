pub fn main() {
    let my_vec: Vec<u8> = vec![0, 1, 2];
    let mut mut_my_vec = my_vec.iter();
    assert_eq!(mut_my_vec.next(), Some(&0));
    assert_eq!(mut_my_vec.next(), Some(&1));
    assert_eq!(mut_my_vec.next(), Some(&2));
    // assert_eq!(mut_my_vec.next(), Some(&3));
    assert_eq!(mut_my_vec.next(), None);
}
