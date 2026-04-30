use std::mem;

#[derive(Debug)]
enum Node {
    Value(i32, Box<Node>),
    Empty,
}

impl Node {
    fn addr(&self) -> *const i32 {
        match self {
            Node::Value(v, _) => v as *const i32,
            Node::Empty => std::ptr::null(),
        }
    }

    fn next(&self) -> &Node {
        match self {
            Node::Value(_, n) => &**n,
            Node::Empty => self,
        }
    }

    fn print_list(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    println!("=== Linked List Middle Node Deletion Proof ===\n");
    let mut list = Node::Value(
        10,
        Box::new(Node::Value(
            20,
            Box::new(Node::Value(
                30,
                Box::new(Node::Value(40, Box::new(Node::Empty))),
            )),
        )),
    );
    println!("Initial List:");
    list.print_list();

    // ==================== Before Deletion ====================
    println!("\nBefore deletion:");
    let a1 = list.addr();
    let a2 = list.next().addr();
    let a3 = list.next().next().addr();
    let a4 = list.next().next().next().addr();
    let a5 = list.next().next().next().next().addr();
    println!("a1 : {:p}", a1);
    println!("a2 : {:p}", a2);
    println!("a3 : {:p}", a3);
    println!("a4 : {:p}", a4);
    println!("a5 : {:p}", a5);

    // ==================== Delete 2nd Node ====================
    println!("\n--- Deleting 2nd Node (20) ---");
    if let Node::Value(_, ref mut next) = list {
        let mut node2 = mem::replace(next, Box::new(Node::Empty));
        if let Node::Value(_, ref mut next_of_2) = *node2 {
            *next = mem::replace(next_of_2, Box::new(Node::Empty));
        }
        drop(node2);
    }

    // ==================== After Deletion ====================
    println!("\nAfter deletion:");
    let b1 = list.addr();
    let b2 = list.next().addr();
    let b3 = list.next().next().addr();
    let b4 = list.next().next().next().addr();
    let b5 = list.next().next().next().next().addr();
    println!("b1 : {:p}", b1);
    println!("b2 : {:p}", b2);
    println!("b3 : {:p}", b3);
    println!("b4 : {:p}", b4);
    println!("b5 : {:p}", b5);

    // ==================== Proof ====================
    println!("\n=== PROOF ===");
    println!("b1 == a1 unchanged : {}", b1 == a1);
    println!("a2 doesnt need to compare");
    println!("b2 == a3 unchanged : {}", b2 == a3);
    println!("b3 == a4 unchanged : {}", b3 == a4);
    println!("b4 == a5 unchanged : {}", b4 == a5);
    println!("\nFinal List:");
    list.print_list();
}
