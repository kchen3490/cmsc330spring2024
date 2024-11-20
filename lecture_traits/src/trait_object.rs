/* 11/20/2024 */

trait Format {
    fn to_s(&self)->String;
}

impl Format for u8 {
    fn to_s(&self) -> String {format!("u8: {}", *self) }
}

impl Format for String {
    fn to_s(&self) -> String {format!("string: {}", *self)}
}

fn do_something<T: Format>(x: T) {
    x.method(); // automatically expands itself to create two do_something for u8 and String 
                // if you have a lot of types though, your code size increases a LOT
}

fn print2(x: Box<dyn Format>) {
    println!("{}", x.to_s());
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    print2(Box::new(x));
    print2(Box::new(y));
}