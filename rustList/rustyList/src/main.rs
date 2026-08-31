use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    n: i32,
    prev: RefCell<Weak<Node>>,        // "weak" back-pointer: doesn't own, avoids a leak
    next: RefCell<Option<Rc<Node>>>,  // "strong" forward-pointer: owns
}

fn main() {
    
    let node1 = Rc::new(Node {
        n: 67,
        prev: RefCell::new(Weak::new()),
        next: RefCell::new(None),
    });

    let node2 = Rc::new(Node {
        n: 2,
        prev: RefCell::new(Weak::new()),
        next: RefCell::new(None),
    });

    // node1->next = node2;
    *node1.next.borrow_mut() = Some(Rc::clone(&node2));
    
    // node2->prev = node1;
    *node2.prev.borrow_mut() = Rc::downgrade(&node1);

    // node2->next = node1;
    *node2.next.borrow_mut() = Some(Rc::clone(&node1));

    // node1->prev = node2;
    *node1.prev.borrow_mut() = Rc::downgrade(&node2);

    println!("node1.n = {}", node1.n);
    println!("node2.n = {}", node2.n);

    // walk node1 -> next -> n
    let next_of_1 = node1.next.borrow();
    println!("node1.next.n = {}", next_of_1.as_ref().unwrap().n);

    // walk node1 -> prev -> n  (upgrade the Weak to use it)
    let prev_of_2 = node2.prev.borrow().upgrade().unwrap();
    println!("node1.prev.n = {}", prev_of_2.n);

    println!("strong count on node1 = {}", Rc::strong_count(&node1));
}