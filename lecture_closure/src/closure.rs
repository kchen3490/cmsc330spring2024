/* 11/18/2024 */

/* To run file, type
    rustc closure.rs
        to create an executable called closure
        
    then type
    ./closure
        to run the executable file */

fn apply<T>(mut f:T, x:i32)->i32 
where T: FnMut(i32)->i32 {
    f(x)
}

fn main() {
    let mut s = String::from("Hello");
    let mut double = |x:i32| {
        s.push_str(" world");
        // let m = s;
        2 * x
    };

    let m = apply(&mut double, 10);
    println!("{}", m);

    let m2 = apply(&mut double, 10);
    println!("{}", m2);

    println!("{}", s);
}