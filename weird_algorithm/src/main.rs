use std::cmp::Ordering;
use std::io;

fn weird(n: i64) -> Vec<i64> {
    match n.cmp ( &1 ) {
        Ordering::Equal => vec![1],
        _other => {
            Vec::from([&[n], weird(if n % 2 == 0 { n / 2 } else { n * 3 + 1 }).as_slice()].concat())
        }
    }
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println! ( "{}", weird ( input.trim ( ).parse ( ).expect ( "Input a number" ) ).iter ( ).fold ( String::new(), |acc, &arg| acc + " " + &arg.to_string ( ) ).trim ( ) );
}
