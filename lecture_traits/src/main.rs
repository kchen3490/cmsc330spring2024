/* 11/18/2024 */

mod trait_object;

/* Traits are like interfaces! */
trait Printable {
    fn stringify(&self)->String; // ->String means fn has to return a string
}

impl Printable for i32 { // for any type i32, 
    fn stringify(&self)->String {
        self.to_string() // remember no semicolon means this is the expression we return
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Printable for Point {
    fn stringify(&self)->String {
        format!("Point: x:{}, y:{}", self.x, self.y) // cool format! for multiple args
    }
}

fn main() {
    println!("{}", 1.stringify());
    
    let p1 = Point{x:10,y:20};
    println!("{}", p1.stringify());

    
}