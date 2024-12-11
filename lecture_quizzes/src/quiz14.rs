// Question 8.3
// fn main(){
//     let mut x = String::from("bye");
//     let y = String::from("farewell");
//     let mut z = &mut x;
//     z = y;
//     println!("{x},{z}");
// }

// Question 8.4
fn main(){
    let x = String::from("hello");
    let mut y = x;
    y.push_str(" world!");
    println!("{x}");
}