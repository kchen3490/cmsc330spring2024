#[derive(Debug)]

/* LEt's create a linked list in Rust! */

/*
struct Node {   // pretend this is in C
    int data,
    Node* next; // needs to be pointer in order to have a fixed size allocated per node
                // otherwise, Node next; could fluctuate in size and cause dangler pointers
}
*/

/* Box can create a pointer */

enum List {     // same thing in Rust as above
    Nil,
    Cons(u32, Box<List>),
}



/*
struct Node {   // pretend this is in C
    int data,
    Node* next; // needs to be pointer in order to have a fixed size allocated per node
                // otherwise, Node next; could fluctuate in size and cause dangler pointers
}
*/

fn main() {
    // let list = Cons(1, Box::new(Nil));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}, \nlen={}", list, list.len());
    
}
