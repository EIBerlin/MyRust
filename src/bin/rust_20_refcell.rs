use std::cell::RefCell;

fn main() {
    let x = RefCell::new(10);

    {
        let mut y = x.borrow_mut();
        *y += 5;
    }

    println!("Value: {:?}", x.borrow());
}
