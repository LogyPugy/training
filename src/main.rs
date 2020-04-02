use std::collections::HashMap;

mod fibonacci;
fn main() {
    // Code 1 : Fibonacci
    println!("Static Fibonacci : ");
    for i in 1..10{
        let fibonacci: u64 = fibonacci::fb(i);
        println!("{} : {}", i, fibonacci);
    }

    // Code 2 : Dynamic Fibonacci
    println!("Dynamic Fibonacci : ");
    let mut map = HashMap::new();
    for i in 1..90{
        let fibonacci: u64 = fibonacci::fb_dynamic(i, &mut map);
        println!("{} : {}", i, fibonacci);
    }
}
