use std::ops::Deref;

struct MBox<T>(T);

impl<T> MBox<T> {
    fn new(x: T) -> MBox<T> {
        MBox(x)
    }
}

struct BBox<T> {
    x: T,
}

impl<T> Deref for MBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MBox<T> {
    fn drop(&mut self) {
        println!("value dropped!")
    }
}

pub fn main() {
    let x = 5;
    {
        let y = MBox::new(x);
        BBox { x: *y };
        println!("value created! y is {:?}", *y);
    }
    println!("end scope!")
}
