fn lifetime_generic<'l>(a: &'l str, _b: &'l str) -> &'l str {
    a
}

pub fn main() {
    let a = "a";
    let b = "a";
    lifetime_generic(a, b);
}
