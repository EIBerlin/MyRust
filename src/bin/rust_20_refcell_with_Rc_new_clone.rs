use std::cell::RefCell;
use std::rc::Rc;

struct Company {
    name: String,
}

fn main() {
    let owner1 = Rc::new(RefCell::new(Company {
        name: String::from("X Co., Ltd"),
    }));

    let owner2 = Rc::clone(&owner1);
    let owner3 = Rc::clone(&owner1);

    {
        let mut company_ref = owner1.borrow_mut();
        company_ref.name = String::from("Z Co., Ltd");
    }

    println!("Owner counts: {}", Rc::strong_count(&owner1));
    println!("Owner 1 sees: {}", owner1.borrow().name);
    println!("Owner 2 sees: {}", owner2.borrow().name);
    println!("Owner 3 sees: {}", owner3.borrow().name);
}
