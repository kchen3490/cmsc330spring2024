/* 11/18/2024 */

// OCaml example
/*
let f x = 
    fun y -> x + y;;

f 10
*/

fn main() {
    let outer = 100;
    
    /* Functions cannot capture outside vars */
    fn next(x:i32)->i32 {
        x + 1 // + outer does not work because outer is not in the environment with x
    }
    println!("{}", next(10));

    /* closures can capture outside vars */
    // next is a closure
    let next = |x:i32|->i32 { //|_| takes in an argument for the closure's environment (in this case outer)
        x + 1 + outer
    };
    println!("{}", next(10));

    /* outer gets shadowed */
    let outer = String::from("hello");

    let next = || { // next takes in no argument, and next gets shadowed
        let m = &outer;
        println!("{}", m);
    };
    next();
    println!("{}", outer);
    next();
}
