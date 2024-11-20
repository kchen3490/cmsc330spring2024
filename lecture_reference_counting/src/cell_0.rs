
ruse std::cell::Cell;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    color: Cell<u32>,
}

fn main() {
    let p1 = Point {
        x: 10,
        y: 20,
        color: Cell::new(88),
    }

    
}