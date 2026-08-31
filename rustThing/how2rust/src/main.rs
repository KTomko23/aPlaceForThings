use std::str;

fn main() 
{
    println!("Hello, world!");



    let x = 10;
    let y = 5;

    let mut v = 5;

    println!("x: {}, y: {}, v: {}", x, y, v);

    let sum = add(x, y);
    println!("Sum: {}", sum);

    let a: u8 = 0b0001_1010;

    println!("a << 1 = {:08b}", a << 1); // 00110100
    println!("a >> 2 = {:08b}", a >> 2); 


    let b: u8 = 0b1011_0110;

    let low_nibble = b & 0b0000_1111;
    let high_nibble = (b & 0b1111_0000) >> 4;

    println!("low  nibble: {:04b}", low_nibble);
    println!("high nibble: {:04b}", high_nibble);

    let value: u32 = 0x12345678;

    let b0 = (value & 0x000000FF) >> 0;
    let b1 = (value & 0x0000FF00) >> 8;
    let b2 = (value & 0x00FF0000) >> 16;
    let b3 = (value & 0xFF000000) >> 24;

    println!("bytes: {:02X} {:02X} {:02X} {:02X}", b3, b2, b1, b0);

    let mut data = [0u8; 4];

    data[0] = 0x12;
    data[1] = 0x34;
    data[2] = 0x56;
    data[3] = 0x78;

    println!("{:02X?}", data);



    //malloc stuff

    //int *b = malloc(sizeof(int));
    let b = Box::new(42);

    //int *arr = malloc(sizeof(int) * n);
    let mut v  = Vec::with_capacity(10);
    v.push(1);
    v.push(2);


    //char *s = malloc(...);
    let s = String::from("m.a.l.l.o.c");

    println!("s: {}",s);

    for i in 0..=10
    {

        println!("{}", i);
    }


    let rustName: &str = "rustacean";
    println!("name = {}", rustName);

    let sum = add(3, 4);
    println!("sum = {}", sum);

    if sum > 5 {
        println!("sum is big");
    } else {
        println!("sum is small");
    }

    for i in 0..3 {
        println!("loop i = {}", i);
    }

    let mut count = 0;
    while count < 2 {
        println!("while count = {}", count);
        count += 1;
    }

    let point = (3, 4);
    println!("point = ({}, {})", point.0, point.1);



    let s1 = String::from("hello");
    takes_ownership(s1);
    //println!("{}", s1);

    let s2 = String::from("hi");
    borrows(&s2);
    println!("s2 still valid: {}", s2);

    let mut s3 = String::from("hello");
    mutate(&mut s3);
    println!("s3 after mutate: {}", s3);

    let a = 5;
    let b = a;
    println!("a = {}, b = {}", a, b);



    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr = {:?}", arr);
    println!("arr[2] = {}", arr[2]);

    //`int *` + malloc/realloc
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("v = {:?}", v);

    //slices: a borrowed VIEW into an array or Vec(ptr, len)
    let slice: &[i32] = &arr[1..4];
    println!("slice = {:?}", slice);

    // whatever garbage byte happens to be past the end of the buffer
    match arr.get(10) {
        Some(val) => println!("got {}", val),
        None => println!("index 10 out of bounds -> None, not garbage memory"),
    }

    let sum: i32 = v.iter().sum();
    println!("sum of v = {}", sum);


    let mut rect = Rectangle::new(3.0, 4.0);
    println!("area = {}", rect.area());

    rect.scale(2.0);
    println!("area after scale = {}", rect.area());

    println!("{}", rect.describe());

    //static dispatch via generics: works for ANY type implementing Shape
    fn print_area<T: Shape>(shape: &T) {
        println!("generic area = {}", shape.area());
    }
    print_area(&rect);


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
    println!("node2.prev.n = {}", prev_of_2.n);

    println!("strong count on node1 = {}", Rc::strong_count(&node1));
}


fn add(a: i32, b: i32) -> i32 {
    a + b
}



fn takes_ownership(s: String) {
    println!("inside function: {}", s);
} // s is dropped here, memory freed automatically

fn borrows(s: &String) {
    println!("borrowed: {}", s);
}

fn mutate(s: &mut String) {
    s.push_str(" world");
}

struct Rectangle {
    width: f64,
    height: f64,
}

// impl blocks attach behavior to a type -- but Rectangle itself
// has NO knowledge of these methods. There's no vtable, no inheritance,
// no implicit "this" pointer with hidden runtime cost.
impl Rectangle {
    // associated function (no self) -- like a "static" method / constructor
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    // method taking &self -- borrows, doesn't consume the struct
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // method taking &mut self -- can mutate
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

// traits are Rust's answer to "interfaces" -- shared behavior across types
// with no inheritance hierarchy at all.
trait Shape {
    fn area(&self) -> f64;
    fn describe(&self) -> String {
        // trait methods can have default implementations
        format!("a shape with area {:.2}", self.area())
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}


use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    n: i32,
    prev: RefCell<Weak<Node>>,        // "weak" back-pointer: doesn't own, avoids a leak
    next: RefCell<Option<Rc<Node>>>,  // "strong" forward-pointer: owns
}
