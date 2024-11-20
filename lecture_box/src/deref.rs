
fn hello(x: &str) {
    println!("hello {}", x);
}

fn main() {
    let m = Box::new(String::from("Rust"));

    let t = *m;

    hello(&m);
    println!("{}",m);
    hello(&(*m)[..]);
}