/* 11/18/2024 */

fn outer(x:i32)->i32 {
    fn inner(y:i32)->i32 { // the nested inner fn
        y * 2
    }
    inner(x) + inner(x) // will return the closure itself
}

fn return_fn(x:i32)->impl Fn(i32) -> i32 {
    // fn inner(y:i32)->i32 {
    //     y * 2
    // }
    /* move keyword moves ownership of y to the y within inner2 */
    let inner2 = move |y:i32| y * 2 + x; // move brings inner to environment
    inner2 // returns the closure itself
}

fn main() {
    let t = outer(20); 
    println!("{t}"); // whoa! this is cool! you can put the variable name within {}
    /* Why did t return 80?
        line 7: inner(x) + inner(x)
        the x's are each 20, given outer(20), which 
        then spit out 40, given inner(y:i32)->i32 returns y * 2 
        so then 40 + 40 */


    let f = return_fn(10); // takes this closure and returns 
    let t2 = f(20);
    println!("t2={t2}");
}

/*
In Java, we have a JavaL Runnable
{
final x = 10;
create a runnable
    y + x;
...
}
// foo returns; x is removed from the stack
...
x = 20;

start runnable; // cannot run code unless x is immutable

later: scheduler will run it
*/