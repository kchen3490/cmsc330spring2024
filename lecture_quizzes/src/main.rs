//Used for question 2, 6
/*
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
*/


fn main() {
    /* // Question 2
    let test = {
        let x = 32;
        let y = true;
        let z = if y{x; 1.0} else {let a = 2.0; a};
        println!("{}", type_of(z));
    };
    //println!("{}", type_of(1.0));
    //println!("{}", type_of(test));  // should return z
    */

    /* // Question 3
    let z = {
        let x = 56;
        let y = 38;
        y = x + 12;
        x + y
    };
    print!("{}", z);
    */

    /* // Question 6
    let mut vec = vec![1, 2, 3];
    for i in &vec {
        println!("{}", i);
    }
    vec.push(4);
    */

    /* // Question 9
    let x = String::from("CMSC");
    let y = &mut x;
    y.push_str("330");
    println!("{}",x);
    */

    /* // Question 10
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
    */

    /*
    // Question 12
    let s1 = String::from("Hello");
    {
        let s2 = &s1;
        println!("{}", s2);
    }

    let s3 = String::from(" World");
    let s4 = s3;

    // println!("{}", s3); // will not work b/c borrow ownership transferred to s4

    let s5 = {
        let mut s6 = String::from("Cliff");
        s6.push_str(" Bakalian");
        s6
    };

    println!("{}", s5);

    let x = 10;
    let y = {
        let z = &x;
        println!("{}", z);
        z
    };

    println!("{}", s4);
    println!("{}", y);
    */

    // Question 11
    let s = String::from("Hello");
    greet(&s);
    println!("{}", s);
    change_string(s);
    println!("{}", s);
}

fn greet(s: &String) {
    println!("Greeting {}", s);
}

fn change_string(s: String) {
    println!("Changed {}", s);
}
