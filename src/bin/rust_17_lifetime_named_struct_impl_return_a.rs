struct MyStruct<'l>(&'l str);

impl<'l> MyStruct<'l> {
    fn lifetime_generic<'x>(&self, a: &'x str, _b: &str) -> &'x str {
        a
    }
}

pub fn main() {
    let a = "a";
    let b = "b";

    let my_struct = MyStruct(&a);
    my_struct.lifetime_generic(a, b);
}
