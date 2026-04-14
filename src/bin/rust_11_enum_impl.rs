enum MyEnum {}

impl MyEnum {
    fn plus(a: i8, b: i8) {
            let c: i8 = a + b;
            println!("{a} + {b} = {c}");
    }
    fn minus(a: i8, b: i8) {
            let c: i8 = a - b;
            println!("{a} - {b} = {c}");
    }
    fn multiply(a: i8, b: i8) {
            let c: i16 = a as i16 * b as i16;
            println!("{a} * {b} = {c}");
    }
}

pub fn main() {
    MyEnum::plus(10, 20);
    MyEnum::minus(10, 20);
    MyEnum::multiply(10, 20);
}