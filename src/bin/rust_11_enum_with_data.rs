#[allow(dead_code)]

enum MyEnum {
    Plus(i8, i8),
    Minus(i8, i8),
    Multiply(i8, i8),
}

pub fn main() {
    let my_plus = MyEnum::Plus(10, 20);
    match my_plus {
        MyEnum::Plus(a, b) => {
            let c: i8 = a + b;
            println!("{a} + {b} = {c}");
        },
        _ => panic!(),
    }
}