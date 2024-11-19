/* 11/18/2024 */

/* Looking through Cliff's notes on Rust */

/*
    To create a new rust project/program, go to terminal and type: 
    cargo new --bin name
    
    
    To build and run rust programs, go to terminal and type
    cargo build

    cargo run

    OR

    type rustc fileName.rs
    to create executable

    then type ./fileName
    to run the executable
    (replace fileName/ .rs program file)
*/

fn main() { // unit is the only time you don't need to specify return values
    println!("Hello, world!");

    let a = [1,2,3,4];
    // this has type [i32;4]. Rust will default to i32 for numbers in that range

    let a = [3;5];
    // this tells Rust to make an arrayof size 5 with each element being the value 3
    // this has type [i32;5]

    let x = String::from("Hello"); //x owns value
    {
        let y = x;
        println!("y is the owner of {}",y);
    };
    println!("x cannot be used here"); // y drops the value, but ownership is NOT passed back to x

    // We can only pass back ownership with return values
    let x = String::from("Hello"); //x owns value
    let y = this(x);
    println!("y is the owner of {}", y);
}

fn this(a:String)->String { // every other fn that doesn't return unit () needs a return type
    a
}
