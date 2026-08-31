// Rust example of nodes being introduced and how to link them
//(repeat purpose for trees etc)
use std::cell::RefCell;
use std::rc::{Rc};
use rand::random;

type Link = Option<Rc<RefCell<Thing>>>;

struct Thing
{
    data: i32,
    prev: Link,
    next: Link,
}


fn main()
{
    println!("\n");

    let mut arr = vec![0i32; 5];

    arr[0] = random::<i32>().abs() % 100;
    arr[1] = random::<i32>().abs() % 100;
    arr[2] = random::<i32>().abs() % 100;
    arr[3] = random::<i32>().abs() % 100;
    arr[4] = random::<i32>().abs() % 100;

    print!("Array: ");
    for i in 0..5
    {
        print!(" {}", arr[i]);
    }
    println!("");

    let mut headThing = Rc::new(RefCell::new(Thing {
        data: 67,
        next: None,
        prev: None,
    }));

    let mut tailThing = headThing.clone();

    for i in 0..5
    {
        let mut newThing = Rc::new(RefCell::new(Thing {
            data: arr[i],
            next: None,
            prev: None,
        }));

        newThing.borrow_mut().prev = Some(tailThing.clone());
        newThing.borrow_mut().next = Some(headThing.clone());

        tailThing.borrow_mut().next = Some(newThing.clone());

        tailThing = newThing.clone();
    }

    headThing.borrow_mut().prev = Some(tailThing.clone());


    //let mut a = headThing.clone();
    print!("List: ");
    for i in 0..6
    {
        let next =
        {
            let a = headThing.borrow();
            print!("{} ", a.data);
            a.next.as_ref().unwrap().clone()
        };

        headThing = next;
    }
    println!();


    //circular
    print!("Circular: ");
    for i in 0..18
    {
        let next =
            {
                let a = headThing.borrow();
                print!("{} ", a.data);
                a.next.as_ref().unwrap().clone()
            };

        headThing = next;
    }
    println!();


    print!("Reverse: ");
    for i in 0..18
    {
        let prev =
            {
                let a = tailThing.borrow();
                print!("{} ", a.data);
                a.prev.as_ref().unwrap().clone()
            };

        tailThing = prev;
    }

    println!();



}
