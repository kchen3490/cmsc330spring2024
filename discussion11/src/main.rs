#![allow(unused_imports)]
#![allow(warnings)]

fn is_prime(n: u32) -> bool {
    if n <= 2 {
        return true; // 1 and 2 are the edge cases for being prime
    } 
    for i in (2..n) { // n should be exclusive and be from 3 to 500
        if n % i == 0 {
            return false; // there is another value which n is divisible by which makes n composite
        }
    }
    true // we have passed all cases and have found no other value from 2 to n-1 that are multiples of n
}

fn main() {
    let mut count = 0;
    for i in (1..101) {
        if is_prime(i) {
            println!("{} is prime", i);
            count += 1;
        } else {
            println!("{} is composite", i);
        }
    }

    println!("We have found {} primes from 1 to 500", count);
}
