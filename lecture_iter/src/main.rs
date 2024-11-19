/* 11/18/2024 */

fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let mut v2 = vec![1,2,3];
    for i in v.iter() {
        println!("{}", i);
    }

    for i in v.iter_mut() {
        *i = &*i * 10; // 2nd value here dereferences a pointer (*) and then borrows it from heap (&)
        //v.push(99); // should not work since we are borrowing a 2nd time
    }
    for i in v.iter() {
        println!("{}", i);
    }

}
