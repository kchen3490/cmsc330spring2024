// enum for question 11
// enum Counter {
//     Node(Option<Box<Counter>>)
// }

// enum and uses for question 12
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // Question 4
    // let mut count = 0;
    // let closure = || {
    //     count += 1;
    //     count
    // };

    // println!("{}", closure());
    // println!("{}", closure());
    // println!("{}", closure());

    // Question 6
    // let x = |a: i32, b: i32| a + b;
    // println!("{}", x(3, 7));

    // Question 7
    // let mut value = 10;
    // let mut closure = |x| {
    //     value += x;
    //     value
    // };

    // println!("{}", closure(5));
    // println!("{}", closure(5));

    // Question 8
    // let mut x = 0;
    // let mut outer = || {
    //     let mut inner = || {
    //         x += 1;
    //         x
    //     };

    //     inner() + inner()
    // };

    // println!("{}", outer());
    // println!("{}", outer());

    // Question 10
    // use std::cell::RefCell;
    // use std::rc::Rc;

    // let x = Rc::new(RefCell::new(5));
    // let y = Rc::clone(&x);
    // let mut borrow1 = x.borrow_mut();
    // let mut borrow2 = x.borrow_mut();

    // Question 11
    // let mut my_list = Counter::Node(Some(Box::new(
    //     Counter::Node(Some(Box::new(
    //         Counter::Node(Some(Box::new(
    //             Counter::Node(Some(Box::new(
    //                 Counter::Node(None)
    //             )))
    //         )))
    //     )))
    // )));

    // loop {
    //     match my_list {
    //         Counter::Node(None) => {
    //             break;
    //         }
    //         Counter::Node(Some(b)) => {
    //             my_list = *b;
    //             println!("1, ");
    //         }
    //     }
    // }

    // Question 12 (print statements are off, except the first two and last one)
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // Mark 1
    println!("Reference count for [5,10] is {}", Rc::strong_count(&a));
    {let c = {
        let b = Rc::new(Cons(4, Rc::clone(&a)));
        // Mark 2
        println!("Reference count for [5, 10] is {}", Rc::strong_count(&b));
        let y = cons(7,&b);
        // Mark 3
        println!("Reference count for [4, 5, 10] is {}", Rc::strong_count(&b));
        y
    };
    if let Cons(_,b) = c{
        // Mark 4
        println!("Reference count for [4, 5, 10] is {}", Rc::strong_count(&b));
    }
    }
    // Mark 5
    println!("Reference count for [5, 10] is {}", Rc::strong_count(&a));
    println!("Reference count for [4, 5, 10] is {}", Rc::strong_count(&a));
}

// fn for question 12
fn cons(y:i32,x:&Rc<List>)->List{
    Cons(y,Rc::clone(&x))
}