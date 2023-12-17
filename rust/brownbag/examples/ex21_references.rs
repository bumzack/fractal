use std::fmt::{Debug, Display};

fn main() {
    let mut s = String::from("Hello world");
    println!("s = {}", s);

    let s2 = &mut s;
    *s2 = String::from("Good by world");

    println!("s2 {}", s2);
}
