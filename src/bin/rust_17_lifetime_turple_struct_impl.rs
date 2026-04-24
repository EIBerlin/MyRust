


struct MyStruct<'l> (&'l str);

impl<'l> MyStruct<'l> {
    fn lifetime_generic(&self, _a: &str, _b: &str) -> &str {
        self.0
    }
}

pub fn main() {
    let a = "a";
    let b = "a";
    let my_struct = MyStruct(&a);
    my_struct.lifetime_generic(a, b);
}
