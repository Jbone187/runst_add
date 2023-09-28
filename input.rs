use std::io;

fn main() {
    let mut input = String::new();

    println!("Welcome to Rust Math");
    println!("");
    println!("Please Enter Numbers to Add By 10");

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let convert_to_int: i64 = input.trim().parse().unwrap();
    let sum = 10;
    let results = convert_to_int + sum;

    println!("\nResults are {results}\n");
}
