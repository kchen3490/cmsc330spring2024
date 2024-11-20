
enum Number {
    Zero,
    One,
    Two,
}

use Number::Zero;

fn main() {
    let t = Number::One;
    match t {
        Zero => println!("0"),
        Number::One => println!("1"),
    }
}

/* Match is very powerful */
/* 
if else ==> match // syntactic sugar

match e with
True => e1
| False => e2

(match 99
(and x y _ 98 a) ()
*/