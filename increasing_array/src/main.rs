use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: i64 = input.trim ( ).parse ( ).expect ( "Input a valid number" );

    println!("Hello, world!");
}
