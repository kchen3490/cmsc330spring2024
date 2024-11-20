
fn main() {
    let v = vec![1,2,3,4,5];
    let s = v.iter().fold(0, |acc, num| acc + num); // sums all nums in v to 0 into s
    println!("Sum= {s}");

    // filter returns an iterator
    let v2 = v.iter().filter(|n| (*n) % 2 == 0);    // new vector w/ items divisible by 2
    print!("Filter: ");
    for i in v2{print!("{i},");}

    print!("\nMap:");
    let v3 = v.iter().map(|n| n * 10);  // applies *10 to all items in v
    for i in v3{print!("{i},");}
    println!();
}