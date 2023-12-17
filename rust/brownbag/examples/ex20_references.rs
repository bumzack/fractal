use std::fmt::{Debug, Display};

fn main() {
    let s = String::from("Hello world");
    println!("s = {}", s);

    print_len(s);

    println!("s {}", s);
}

fn print_len(s: String) {
    println!("length of string '{}' = {}", s, s.len());
}
