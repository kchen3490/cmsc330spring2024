/*
* First time learning Rust!! - Nov 11, 2024
*/

fn main() {
    //let x:i32 = 10; // signed integer, can be either +/-

    //let x:u32 = -1; // unsigned integer, cannot have unary operator `-`

    let x: i32 = 10;    // immutable
    let x: i32 = 11;    // shadowing, so is okay
    // x = 11;          // not okay, because x is immutable

    println!("x is {}", x);

    let y:i32 = x;      // Copy
    println!("y is {}", y);
    /*
    Copy: assignment will clone, (e.g., y points to copy of 10, 
        x & y point to two separate points of memory, each carrying a value of 10)
    */
    
    /*
    Move: assignment will transfer ownership (example below)
    */
    let s1 = String::from("hello"); // "hello" is created as String obj on heap
    {
        let s2 = s1;            // s1 lost the ownership
        //println!("{}", s1);   // error
    } // s2 is out of scope. s2 will clean the memory for "hello".
    //println!("{}", s1);       // error 
    
    /*
    Mutability and borrow
    */
    let mut s1: String = String::from("hello");
    {
        let s2: &String = &s1;       // s2 borrows s1 using &
        //s1.push_str(string: " world"); // cannot mutate during a borrow
        println!("s2 is {}", s2);
        println!("s1 is {}", s1);   // should also work!

    }   // give up the borrow
    
    s1.push_str(" world");
    println!("{}", s1);

    /*
    More mutability stuff
     */
    let mut s1: String = String::from("hello");

    /*
    While s2 borrows s1, s1 cannot do anything until the borrow
    is done. Imagine lending your bank account to your friend,
    your friend borrows $1000, and you can't do anything about
    interaction until the borrow is over.
     */
    {
        // s2 can also be a mutated borrow
        let mut s2 = &mut s1; // s2 borrows s1
        println!("{}", s2);

        s2.push_str(" world 1");
        // s2 can mutate, but
        // s1 cannot mutate

        // s2 can borrow something else and let go of the original borrow
        s2 = &mut String::from("alice");

        // so s1 can now modify itself
        s1.push_str(" world 2");

        // s2 and s1 point to the same obj "hello world 1"
        println!("{}", s1);

    }
}
