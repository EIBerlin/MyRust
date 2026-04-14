enum Calc {
    Plus(i8, i8),
    Minus(i8, i8),
}

impl Calc {
    fn exec(&self) {
        match self {
            Calc::Plus(a, b) => {
                let c: i8 = a + b;
                println!("{a} + {b} = {c}");
            },
            Calc::Minus(a, b) => {
                let c: i8 = a - b;
                println!("{a} - {b} = {c}");
            },
        }
    }
}

pub fn main() {
    Calc::Minus(10, 20).exec();
    Calc::Plus(10, 20).exec();
}