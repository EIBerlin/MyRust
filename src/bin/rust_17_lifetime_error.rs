fn lifetime_error(a: &str, b: &str) -> &str {
    /*expected named lifetime parameter*/
    a
}

pub fn main() {
    let a: &str = "a";
    let b: &str = "a";
    lifetime_error(&a, &b);
}
