#[derive(Debug)]
enum MySingleLinkedList<T> {
    Node {
        value: T,
        next: Box<MySingleLinkedList<T>>,
    },
    Null,
}

pub fn main() {
    let my_single_linked_list = MySingleLinkedList::Node {
        value: 12,
        next: Box::new(MySingleLinkedList::Node {
            value: 30,
            next: Box::new(MySingleLinkedList::Node {
                value: 40,
                next: Box::new(MySingleLinkedList::Null),
            }),
        }),
    };

    println!("{:#?}", my_single_linked_list);
}
